#[derive(Debug)]
pub struct GanttChart {
    pub segments: Vec<GanttSegment>,
}

#[derive(Debug)]
pub struct GanttSegment {
    pub pid: usize,
    pub start_time: u32,
    pub end_time: u32,
}

impl GanttChart {
    pub fn new() -> Self {
        GanttChart {
            segments: Vec::new(),
        }
    }

    pub fn display_gantt_chart(&self) {
        if self.segments.is_empty() {
            return;
        }

        println!("\n{:=^90}", " GANTT CHART ");

        // Print top border
        print!("\n");
        for segment in &self.segments {
            let width = (segment.end_time - segment.start_time) as usize * 4;
            print!("+{}", "-".repeat(width));
        }
        println!("+");

        // Print process IDs
        print!("|");
        for segment in &self.segments {
            let width = (segment.end_time - segment.start_time) as usize * 4;
            let label = format!("P{}", segment.pid);
            let padding = (width - label.len()) / 2;
            print!(
                "{}{}{}",
                " ".repeat(padding),
                label,
                " ".repeat(width - padding - label.len())
            );
            print!("|");
        }
        println!();

        // Print bottom border
        for segment in &self.segments {
            let width = (segment.end_time - segment.start_time) as usize * 4;
            print!("+{}", "-".repeat(width));
        }
        println!("+");

        // Print time labels
        print!("{}", self.segments[0].start_time);
        for segment in &self.segments {
            let width = (segment.end_time - segment.start_time) as usize * 4;
            let time_label = segment.end_time.to_string();
            print!("{}{}", " ".repeat(width - time_label.len() + 1), time_label);
        }
        println!("\n");
    }
}
