use std::collections::HashMap;

use niri_ipc::{Action, Request, Response, socket};

const THRESHOLD: f64 = 1.;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = socket::Socket::connect()?;

    let Response::Windows(s) = socket.send(Request::Windows)?? else {
        panic!("compositor did not respond with the correct response code");
    };

    let mut prev_windows: HashMap<u64, (f64, f64)> =
        s.into_iter().map(|x| (x.id, x.layout.tile_size)).collect();

    let Response::Handled = socket.send(Request::EventStream)?? else {
        panic!("compositor did not respond with the correct response code");
    };

    let mut get_events = socket.read_events();

    let mut i = 0;
    loop {
        match get_events() {
            Ok(event) => {
                i += 1;
                match event {
                    niri_ipc::Event::WindowLayoutsChanged { changes } => {
                        for (id, layout) in changes {
                            if let Some(&(w1, h1)) = prev_windows.get(&id)
                                && (w1 - layout.tile_size.0).abs() > THRESHOLD
                            {
                                let mut socket = socket::Socket::connect()?;
                                socket.send(Request::Action(Action::CenterVisibleColumns {}))??;
                            }

                            prev_windows
                                .entry(id)
                                .and_modify(|x| *x = layout.tile_size)
                                .or_insert(layout.tile_size);
                        }
                        {
                            let mut p = prev_windows.iter().collect::<Vec<_>>();
                            p.sort_by_key(|x| x.0);
                            dbg!(i, p);
                        }
                    }
                    niri_ipc::Event::WindowOpenedOrChanged { window } => {
                        prev_windows.insert(window.id, window.layout.tile_size);
                        dbg!(i);
                    }
                    niri_ipc::Event::WindowsChanged { windows } => {
                        for window in &windows {
                            if let Some(&(w1, h1)) = prev_windows.get(&window.id)
                                && (w1 - window.layout.tile_size.0).abs() > THRESHOLD
                            {
                                let mut socket = socket::Socket::connect()?;
                                socket.send(Request::Action(Action::CenterVisibleColumns {}))??;
                            }
                        }
                        prev_windows = windows
                            .into_iter()
                            .map(|x| (x.id, x.layout.tile_size))
                            .collect();
                        {
                            let mut p = prev_windows.iter().collect::<Vec<_>>();
                            p.sort_by_key(|x| x.0);
                            dbg!(i, p);
                        }
                    }
                    _ => {}
                }
            }
            Err(x) => {
                eprintln!("{x:#?}");
            }
        }
    }

    Ok(())
}
