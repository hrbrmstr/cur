use std::net::UdpSocket;
use pancurses::{initscr, endwin, noecho};

use cur::{MAX_DATAGRAM_SIZE, Observation};

fn main() {

	let window = initscr();
	window.nodelay(true);

	let mut buf = [0; MAX_DATAGRAM_SIZE];

  let s = UdpSocket::bind(format!("0.0.0.0:{}", 50222))
	  .expect(r#"{"message":"Could not bind to address/port."}"#);

	window.refresh();
	window.keypad(true);

	noecho();

  while window.getch().is_none() {

    let (n, _) = s.recv_from(&mut buf)
      .expect(r#"{"message":"No broadcasts received."}"#);

    let msg = String::from_utf8(buf[..n].to_vec()).expect(r#"Unexpected string processing error."#);

    if msg.contains("obs_st") {
			
			let model: Observation = serde_json::from_str(&msg).expect(r#"Something is very wrong in UDP JSON land."#);
			// https://weatherflow.github.io/Tempest/api/udp/v171/

			let _timestamp = model.obs[0][0];
			let _pressure = model.obs[0][6];
			let temperature = model.obs[0][7];
			let humidity = model.obs[0][8];
			let illuminance = model.obs[0][9];
			let _uv = model.obs[0][10];
			let _solar_radiation = model.obs[0][11];

			let f_str = format!("{:3.1}°F | ", temperature * (9.0 / 5.0) + 32.0);
			let h_str = format!(   "{:3}% | ", humidity as u8);
			let i_str = format!("{:5} lux | ", illuminance as u32);

      window.mvaddstr(0, 0, f_str);
			window.addstr(h_str);
			window.addstr(i_str);
		}

		
	}

	endwin();

}