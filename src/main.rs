/*
    application name: Cpu-ID_fltk.exe
    Author: Abraham Chen
    Date: April, 23 2022
    Description: a system monitor program
*/

#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

extern crate systemstat;

use std::{time::Duration, thread};
use fltk::app::{Screen, windows, set_frame_border_radius_max};
use fltk::{ prelude::*, window::Window, *, frame::Frame, app::channel, misc::Progress, enums::{Align, Color}};
use systemstat::{System, Platform, saturating_sub_bytes};

const MAINW_WIDTH: i32= 350; 
const MAINW_HEIGHT: i32= 100; 

const WIDGET_WIDTH: i32 = 400;
const WIDGET_HEIGHT: i32 = 400;

const WIDGET_PADDING: i32 = 10;
const WIDGET_LABEL_WIDTH: i32 = 110;

const RED: Color = Color::from_rgb(217,4,16);
const  GRAY: Color = Color::from_rgb(138, 143, 144);


enum Message {
    Tick,
}

fn main() {
    let _SCREEN_HEIGHT: i32 = Screen::h(&Screen { n: (0) });
    let _SCREEN_WIDTH: i32 = Screen::w(&Screen { n: (0) });
    let _SCREEN_BORDER: i32 = 50;

    app::background(38, 38, 38);
    app::set_visible_focus(true);
    

    let sys = System::new();

    let app = app::App::default();
    
    let mut window = Window::default()
        .with_size(
            MAINW_WIDTH,
            MAINW_HEIGHT,
        )
        .with_label("CPU_ID")
        .with_pos((_SCREEN_WIDTH - MAINW_WIDTH) / 2, 0);

    let (sender, reciever) = channel::<Message>();

    thread::spawn(move || loop{
        thread::sleep(Duration::from_millis(100));
        sender.send(Message::Tick);
    });

    /** Progess */
    let mut CPU_LOAD = Progress::default()
        .with_size(WIDGET_WIDTH/2, WIDGET_HEIGHT/32)
        .with_pos(WIDGET_LABEL_WIDTH, WIDGET_PADDING)
        .with_align(Align::Left)
        .with_label("CPU LOAD:");
        
        CPU_LOAD.set_label_color(GRAY);
        CPU_LOAD.set_selection_color(RED);
        CPU_LOAD.set_frame(enums::FrameType::GtkRoundDownFrame);

    let mut RAM_LOAD = Progress::default()
        .with_size(WIDGET_WIDTH/2, WIDGET_HEIGHT/32)
        .below_of(&CPU_LOAD, 2)
        .with_align(Align::Left)
        .with_label("RAM LOAD:");

        RAM_LOAD.set_label_color(GRAY);
        RAM_LOAD.set_selection_color(RED);
        RAM_LOAD.set_frame(enums::FrameType::GtkRoundDownFrame);

    let mut DISK_LOAD = Progress::default()
        .with_size(WIDGET_WIDTH/2, WIDGET_HEIGHT/32)
        .below_of(&RAM_LOAD, 2)
        .with_align(Align::Left)
        .with_label("HDD LOAD");
        
        DISK_LOAD.set_label_color(GRAY);
        DISK_LOAD.set_selection_color(RED);
        DISK_LOAD.set_frame(enums::FrameType::GtkRoundDownFrame);
        
    /** Label */
    let mut CPU_LOAD_AVEGARE = Frame::default()
        .with_size(WIDGET_WIDTH/2, WIDGET_HEIGHT/32)
        .below_of(&DISK_LOAD, WIDGET_PADDING)
        .with_pos(WIDGET_PADDING + WIDGET_LABEL_WIDTH, WIDGET_PADDING*6)
        .with_align(Align::Left)
        .with_label("-");
        CPU_LOAD_AVEGARE.set_label_color(GRAY);
    
    let mut BATTERY_LOAD_AVEGARE = Frame::default()
        .with_size(WIDGET_WIDTH/2, WIDGET_HEIGHT/32)
        .below_of(&CPU_LOAD_AVEGARE, WIDGET_PADDING)
        .with_align(Align::Left)
        .with_label("-");
        BATTERY_LOAD_AVEGARE.set_label_color(GRAY);
    
    let mut RAM_LOAD_AVERAGE = Frame::default()
        .with_size(WIDGET_WIDTH/2, WIDGET_HEIGHT/32)
        .right_of(&CPU_LOAD_AVEGARE, WIDGET_PADDING)
        .with_label("-")
        .with_align(Align::Left);
        RAM_LOAD_AVERAGE.set_label_color(GRAY);

    let mut DISK_LOAD_AVERAGE = Frame::default()
        .with_size(WIDGET_WIDTH/2, WIDGET_HEIGHT/32)
        .below_of(&RAM_LOAD_AVERAGE, WIDGET_PADDING)
        .with_label("-")
        .with_align(Align::Left);
        DISK_LOAD_AVERAGE.set_label_color(GRAY);

    window.end();
    window.set_border(false);
    window.show();

    /** Calculation STUFF */
    while app.wait() {
        match reciever.recv() {
            Some(Message::Tick) => {
                match sys.cpu_load_aggregate() {
                    Ok(cpu) => {
                        thread::sleep(Duration::from_secs(2));
                        let cpu = cpu.done().unwrap();
                        let cpu_load = ((cpu.system + cpu.user + cpu.nice) * 100.0).ceil();
                        
                        CPU_LOAD.set_value(cpu_load.into());
                        CPU_LOAD_AVEGARE.set_label(&format!("CPU Load: {:.0} %", cpu_load));
                        
                    },
                    Err(x) => println!("Error{}", x)   
                }               

                match sys.memory() {
                    Ok(mem) => {
                        let ram_load = saturating_sub_bytes(mem.total, mem.free);
                        let ram_loadf64 :f64;
                        let ram_loadu64 = ram_load.as_u64();
                        ram_loadf64 = ram_loadu64 as f64;

                        let total_ramf64 :f64;
                        let total_ramu64 = mem.total.as_u64();
                        total_ramf64 = total_ramu64 as f64;
                        
                        RAM_LOAD.set_maximum(total_ramf64);
                        RAM_LOAD.set_value(ram_loadf64);
                        RAM_LOAD_AVERAGE.set_label(&format!("RAM Load: {:.3} GB / {:.3} GB", ram_load, mem.total));

                    },
                    Err(x) => println!("Error{}", x) 
                }

                match sys.mounts() {
                    Ok(mounts) => {
                        let total_diskf64 :f64;
                        let total_disku64 = mounts[0].total.as_u64();
                        total_diskf64 = total_disku64 as f64;

                        let disk_loadf64 :f64;
                        let disk_loadu64 = mounts[0].avail.as_u64();
                        disk_loadf64 = disk_loadu64 as f64;

                        DISK_LOAD.set_value(disk_loadf64);
                        DISK_LOAD.set_maximum(total_diskf64);
                        DISK_LOAD_AVERAGE.set_label(&format!("DISK: {:.5} GB / {:.5} GB", mounts[0].avail, mounts[0].total)); 
                    },
                    Err(x) => println!("Error{}", x)  
                }

                match sys.battery_life() {
                    Ok(battery) => {
                        let total_battery = battery.remaining_capacity * 100.0;
                        BATTERY_LOAD_AVEGARE.set_label(&format!("BATTERY: {:.0} %", total_battery))
                    },
                    Err(x) => println!("Error{}", x)
                }
            }
            None => {}
        } 
    }
    app.run().unwrap();
}