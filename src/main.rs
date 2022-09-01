use std::time::Duration;

use eframe::egui;
use parking_lot::deadlock;

struct DeadlockApp {}

impl DeadlockApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        let egui_context = cc.egui_ctx.clone();
        std::thread::spawn(move || -> () {
            loop {
                // Usually, the thread would do some other work and request a repaint less frequently,
                // but the problem would still occur, only less frequently
                std::thread::sleep(Duration::from_millis(5));
                println!("requesting repaint");
                egui_context.request_repaint();
                println!("requesting repaint done");
            }
        });
        Self {}
    }
}

impl eframe::App for DeadlockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("side panel");
        egui::SidePanel::left("side panel").show(ctx, |_| {});
        println!("central panel");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Move mouse over window to cause UI updates!")
            ui.label(format!("{:?}", std::time::Instant::now()));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    // Create a background thread which checks for deadlocks every 1s
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));
        let deadlocks = deadlock::check_deadlock();
        if deadlocks.is_empty() {
            continue;
        }

        println!("{} deadlocks detected", deadlocks.len());
        for (i, threads) in deadlocks.iter().enumerate() {
            println!("Deadlock #{}", i);
            for t in threads {
                println!("Thread Id {:#?}", t.thread_id());
                println!("{:#?}", t.backtrace());
            }
        }
    });

    eframe::run_native(
        "DeadlockApp",
        options,
        Box::new(move |cc| Box::new(DeadlockApp::new(cc))),
    );
}
