use std::thread;
use std::time::Duration;

use dsmr_parse::Telegram;
use prometheus::{Encoder, Gauge, IntGauge, TextEncoder};
use tiny_http::{Header, Response, Server};

struct Metrics {
    electricity_consumed_tariff_1: Gauge,
    electricity_consumed_tariff_2: Gauge,
    electricity_generated_tariff_1: Gauge,
    electricity_generated_tariff_2: Gauge,
    power: Gauge,
    return_power: Gauge,
    power_failure_count: IntGauge,
    long_power_failure_count: IntGauge,
    voltage_sag_l1_count: IntGauge,
    voltage_sag_l2_count: IntGauge,
    voltage_sag_l3_count: IntGauge,
    voltage_swell_l1_count: IntGauge,
    voltage_swell_l2_count: IntGauge,
    voltage_swell_l3_count: IntGauge,
    voltage_l1: Gauge,
    voltage_l2: Gauge,
    voltage_l3: Gauge,
    current_l1: Gauge,
    current_l2: Gauge,
    current_l3: Gauge,
    power_l1: Gauge,
    power_l2: Gauge,
    power_l3: Gauge,
    return_power_l1: Gauge,
    return_power_l2: Gauge,
    return_power_l3: Gauge,
    gas_consumed: Gauge,
}

impl Metrics {
    fn register() -> Self {
        Self {
            electricity_consumed_tariff_1: prometheus::register_gauge!(
                "electricity_consumed_tariff_1_kwh",
                "Total electricity consumed on tariff 1 in kWh"
            )
            .unwrap(),
            electricity_consumed_tariff_2: prometheus::register_gauge!(
                "electricity_consumed_tariff_2_kwh",
                "Total electricity consumed on tariff 2 in kWh"
            )
            .unwrap(),
            electricity_generated_tariff_1: prometheus::register_gauge!(
                "electricity_generated_tariff_1_kwh",
                "Total electricity generated on tariff 1 in kWh"
            )
            .unwrap(),
            electricity_generated_tariff_2: prometheus::register_gauge!(
                "electricity_generated_tariff_2_kwh",
                "Total electricity generated on tariff 2 in kWh"
            )
            .unwrap(),
            power: prometheus::register_gauge!(
                "power_watts",
                "Current power consumption in W"
            )
            .unwrap(),
            return_power: prometheus::register_gauge!(
                "return_power_watts",
                "Current power delivery (solar/generation) in W"
            )
            .unwrap(),
            power_failure_count: prometheus::register_int_gauge!(
                "power_failures_total",
                "Number of power failures"
            )
            .unwrap(),
            long_power_failure_count: prometheus::register_int_gauge!(
                "long_power_failures_total",
                "Number of long power failures"
            )
            .unwrap(),
            voltage_sag_l1_count: prometheus::register_int_gauge!(
                "voltage_sags_l1_total",
                "Number of voltage sags on L1"
            )
            .unwrap(),
            voltage_sag_l2_count: prometheus::register_int_gauge!(
                "voltage_sags_l2_total",
                "Number of voltage sags on L2"
            )
            .unwrap(),
            voltage_sag_l3_count: prometheus::register_int_gauge!(
                "voltage_sags_l3_total",
                "Number of voltage sags on L3"
            )
            .unwrap(),
            voltage_swell_l1_count: prometheus::register_int_gauge!(
                "voltage_swells_l1_total",
                "Number of voltage swells on L1"
            )
            .unwrap(),
            voltage_swell_l2_count: prometheus::register_int_gauge!(
                "voltage_swells_l2_total",
                "Number of voltage swells on L2"
            )
            .unwrap(),
            voltage_swell_l3_count: prometheus::register_int_gauge!(
                "voltage_swells_l3_total",
                "Number of voltage swells on L3"
            )
            .unwrap(),
            voltage_l1: prometheus::register_gauge!(
                "voltage_l1_volts",
                "Instantaneous voltage on L1 in V"
            )
            .unwrap(),
            voltage_l2: prometheus::register_gauge!(
                "voltage_l2_volts",
                "Instantaneous voltage on L2 in V"
            )
            .unwrap(),
            voltage_l3: prometheus::register_gauge!(
                "voltage_l3_volts",
                "Instantaneous voltage on L3 in V"
            )
            .unwrap(),
            current_l1: prometheus::register_gauge!(
                "current_l1_amperes",
                "Instantaneous current on L1 in A"
            )
            .unwrap(),
            current_l2: prometheus::register_gauge!(
                "current_l2_amperes",
                "Instantaneous current on L2 in A"
            )
            .unwrap(),
            current_l3: prometheus::register_gauge!(
                "current_l3_amperes",
                "Instantaneous current on L3 in A"
            )
            .unwrap(),
            power_l1: prometheus::register_gauge!(
                "power_l1_watts",
                "Instantaneous power consumption on L1 in W"
            )
            .unwrap(),
            power_l2: prometheus::register_gauge!(
                "power_l2_watts",
                "Instantaneous power consumption on L2 in W"
            )
            .unwrap(),
            power_l3: prometheus::register_gauge!(
                "power_l3_watts",
                "Instantaneous power consumption on L3 in W"
            )
            .unwrap(),
            return_power_l1: prometheus::register_gauge!(
                "return_power_l1_watts",
                "Instantaneous power delivery on L1 in W"
            )
            .unwrap(),
            return_power_l2: prometheus::register_gauge!(
                "return_power_l2_watts",
                "Instantaneous power delivery on L2 in W"
            )
            .unwrap(),
            return_power_l3: prometheus::register_gauge!(
                "return_power_l3_watts",
                "Instantaneous power delivery on L3 in W"
            )
            .unwrap(),
            gas_consumed: prometheus::register_gauge!(
                "gas_consumed_m3",
                "Total gas consumed in m³"
            )
            .unwrap(),
        }
    }

    fn update(&self, telegram: &Telegram) {
        if let Some(v) = &telegram.electricity_consumed_tariff_1 {
            self.electricity_consumed_tariff_1.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.electricity_consumed_tariff_2 {
            self.electricity_consumed_tariff_2.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.electricity_generated_tariff_1 {
            self.electricity_generated_tariff_1.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.electricity_generated_tariff_2 {
            self.electricity_generated_tariff_2.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.power {
            self.power.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.return_power {
            self.return_power.set(v.value * 1000.0);
        }
        if let Some(v) = telegram.power_failure_count {
            self.power_failure_count.set(i64::from(v));
        }
        if let Some(v) = telegram.long_power_failure_count {
            self.long_power_failure_count.set(i64::from(v));
        }
        if let Some(v) = telegram.voltage_sag_l1_count {
            self.voltage_sag_l1_count.set(i64::from(v));
        }
        if let Some(v) = telegram.voltage_sag_l2_count {
            self.voltage_sag_l2_count.set(i64::from(v));
        }
        if let Some(v) = telegram.voltage_sag_l3_count {
            self.voltage_sag_l3_count.set(i64::from(v));
        }
        if let Some(v) = telegram.voltage_swell_l1_count {
            self.voltage_swell_l1_count.set(i64::from(v));
        }
        if let Some(v) = telegram.voltage_swell_l2_count {
            self.voltage_swell_l2_count.set(i64::from(v));
        }
        if let Some(v) = telegram.voltage_swell_l3_count {
            self.voltage_swell_l3_count.set(i64::from(v));
        }
        if let Some(v) = &telegram.voltage_l1 {
            self.voltage_l1.set(v.value);
        }
        if let Some(v) = &telegram.voltage_l2 {
            self.voltage_l2.set(v.value);
        }
        if let Some(v) = &telegram.voltage_l3 {
            self.voltage_l3.set(v.value);
        }
        if let Some(v) = &telegram.current_l1 {
            self.current_l1.set(f64::from(v.value));
        }
        if let Some(v) = &telegram.current_l2 {
            self.current_l2.set(f64::from(v.value));
        }
        if let Some(v) = &telegram.current_l3 {
            self.current_l3.set(f64::from(v.value));
        }
        if let Some(v) = &telegram.power_l1 {
            self.power_l1.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.power_l2 {
            self.power_l2.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.power_l3 {
            self.power_l3.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.return_power_l1 {
            self.return_power_l1.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.return_power_l2 {
            self.return_power_l2.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.return_power_l3 {
            self.return_power_l3.set(v.value * 1000.0);
        }
        if let Some(v) = &telegram.gas_consumed {
            self.gas_consumed.set(v.value);
        }
    }
}

fn main() {
    let metrics = Metrics::register();

    thread::spawn(move || {
        let mut port = serialport::new("/dev/ttyUSB0", 115_200)
            .timeout(Duration::from_secs(30))
            .open()
            .unwrap_or_else(|e| {
                eprintln!("Failed to open /dev/ttyUSB0: {e}");
                std::process::exit(1);
            });

        loop {
            match Telegram::read_from(&mut port) {
                Ok(Some(telegram)) => {
                    metrics.update(&telegram);
                }
                Ok(None) => {
                    eprintln!("Incomplete telegram received, retrying...");
                }
                Err(e) => {
                    eprintln!("Parse error: {e}");
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    });

    let server = Server::http("0.0.0.0:9091").unwrap_or_else(|e| {
        eprintln!("Failed to start HTTP server: {e}");
        std::process::exit(1);
    });

    println!("Listening on http://0.0.0.0:9091/metrics");

    for request in server.incoming_requests() {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        let content_type =
            Header::from_bytes("Content-Type", encoder.format_type().as_bytes()).unwrap();
        let response = Response::from_data(buffer).with_header(content_type);
        if let Err(e) = request.respond(response) {
            eprintln!("Failed to send response: {e}");
        }
    }
}
