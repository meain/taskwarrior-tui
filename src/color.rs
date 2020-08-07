use std::process::Command;
use std::str;
use tui::style::Color;

#[derive(Debug)]
pub struct TColor {
    pub fg: Color,
    pub bg: Color,
}

impl TColor {
    pub fn default() -> Self {
        TColor {
            fg: Color::Indexed(0),
            bg: Color::Indexed(15),
        }
    }
}

#[derive(Debug)]
pub struct TColorConfig {
    pub enabled: bool,
    pub active: TColor,
    pub alternate: TColor,
    pub blocked: TColor,
    pub blocking: TColor,
    pub burndown_done: TColor,
    pub burndown_pending: TColor,
    pub burndown_started: TColor,
    pub calendar_due: TColor,
    pub calendar_due_today: TColor,
    pub calendar_holiday: TColor,
    pub calendar_overdue: TColor,
    pub calendar_today: TColor,
    pub calendar_weekend: TColor,
    pub calendar_weeknumber: TColor,
    pub completed: TColor,
    pub debug: TColor,
    pub deleted: TColor,
    pub due: TColor,
    pub due_today: TColor,
    pub error: TColor,
    pub footnote: TColor,
    pub header: TColor,
    pub history_add: TColor,
    pub history_delete: TColor,
    pub history_done: TColor,
    pub label: TColor,
    pub label_sort: TColor,
    pub overdue: TColor,
    pub project: TColor,
    pub recurring: TColor,
    pub scheduled: TColor,
    pub summary_background: TColor,
    pub summary_bar: TColor,
    pub sync_added: TColor,
    pub sync_changed: TColor,
    pub sync_rejected: TColor,
    pub tag_next: TColor,
    pub tag: TColor,
    pub tagged: TColor,
    pub uda_priority: TColor,
    pub uda_priority_h: TColor,
    pub uda_priority_l: TColor,
    pub uda_priority_m: TColor,
    pub undo_after: TColor,
    pub undo_before: TColor,
    pub until: TColor,
    pub warning: TColor,
}

pub fn get_color(s: &str) -> Color {
    if s.starts_with("color") {
        let fg = (s.as_bytes()[5] as char).to_digit(10).unwrap() as u8;
        Color::Indexed(fg)
    } else if s.starts_with("rgb") {
        let red = (s.as_bytes()[3] as char).to_digit(10).unwrap() as u8;
        let green = (s.as_bytes()[4] as char).to_digit(10).unwrap() as u8;
        let blue = (s.as_bytes()[5] as char).to_digit(10).unwrap() as u8;
        Color::Indexed(16 + red * 36 + green * 6 + blue)
    } else {
        if s == "white" {
            Color::White
        } else if s == "black" {
            Color::Black
        } else {
            Color::Indexed(15)
        }
    }
}

pub fn get_tcolor(line: &str) -> TColor {
    if line.contains(" on ") {
        let foreground = line.split(" ").collect::<Vec<&str>>()[0];
        let background = line.split(" ").collect::<Vec<&str>>()[2];
        TColor {
            fg: get_color(foreground),
            bg: get_color(background),
        }
    } else if line.contains("on ") {
        let background = line.split(" ").collect::<Vec<&str>>()[1];
        TColor {
            fg: Color::Indexed(0),
            bg: get_color(background),
        }
    } else {
        let foreground = line;
        TColor {
            fg: get_color(foreground),
            bg: Color::Indexed(15),
        }
    }
}

impl TColorConfig {
    pub fn default() -> Self {
        let output = Command::new("task")
            .arg("rc.color=off")
            .arg("show")
            .output()
            .expect("Unable to run `task show`");

        let data = String::from_utf8(output.stdout).expect("Unable to convert stdout to string");

        let enabled = true;
        let mut active = TColor::default();
        let mut alternate = TColor::default();
        let mut blocked = TColor::default();
        let mut blocking = TColor::default();
        let mut burndown_done = TColor::default();
        let mut burndown_pending = TColor::default();
        let mut burndown_started = TColor::default();
        let mut calendar_due = TColor::default();
        let mut calendar_due_today = TColor::default();
        let mut calendar_holiday = TColor::default();
        let mut calendar_overdue = TColor::default();
        let mut calendar_today = TColor::default();
        let mut calendar_weekend = TColor::default();
        let mut calendar_weeknumber = TColor::default();
        let mut completed = TColor::default();
        let mut debug = TColor::default();
        let mut deleted = TColor::default();
        let mut due = TColor::default();
        let mut due_today = TColor::default();
        let mut error = TColor::default();
        let mut footnote = TColor::default();
        let mut header = TColor::default();
        let mut history_add = TColor::default();
        let mut history_delete = TColor::default();
        let mut history_done = TColor::default();
        let mut label = TColor::default();
        let mut label_sort = TColor::default();
        let mut overdue = TColor::default();
        let mut project = TColor::default();
        let mut recurring = TColor::default();
        let mut scheduled = TColor::default();
        let mut summary_background = TColor::default();
        let mut summary_bar = TColor::default();
        let mut sync_added = TColor::default();
        let mut sync_changed = TColor::default();
        let mut sync_rejected = TColor::default();
        let mut tag_next = TColor::default();
        let mut tag = TColor::default();
        let mut tagged = TColor::default();
        let mut uda_priority = TColor::default();
        let mut uda_priority_h = TColor::default();
        let mut uda_priority_l = TColor::default();
        let mut uda_priority_m = TColor::default();
        let mut undo_after = TColor::default();
        let mut undo_before = TColor::default();
        let mut until = TColor::default();
        let mut warning = TColor::default();

        for line in data.split('\n') {
            if line.starts_with("color.active ") {
                active = get_tcolor(
                    line.trim_start_matches("color.active ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.alternate ") {
                alternate = get_tcolor(
                    line.trim_start_matches("color.alternate ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.blocked ") {
                blocked = get_tcolor(
                    line.trim_start_matches("color.blocked ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.blocking ") {
                blocking = get_tcolor(
                    line.trim_start_matches("color.blocking ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.burndown.done ") {
                burndown_done = get_tcolor(
                    line.trim_start_matches("color.burndown.done ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.burndown.pending ") {
                burndown_pending = get_tcolor(
                    line.trim_start_matches("color.burndown.pending ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.burndown.started ") {
                burndown_started = get_tcolor(
                    line.trim_start_matches("color.burndown.started ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.calendar.due ") {
                calendar_due = get_tcolor(
                    line.trim_start_matches("color.calendar.due ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.calendar.due.today ") {
                calendar_due_today = get_tcolor(
                    line.trim_start_matches("color.calendar.due.today ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.calendar.holiday ") {
                calendar_holiday = get_tcolor(
                    line.trim_start_matches("color.calendar.holiday ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.calendar.overdue ") {
                calendar_overdue = get_tcolor(
                    line.trim_start_matches("color.calendar.overdue ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.calendar.today ") {
                calendar_today = get_tcolor(
                    line.trim_start_matches("color.calendar.today ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.calendar.weekend ") {
                calendar_weekend = get_tcolor(
                    line.trim_start_matches("color.calendar.weekend ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.calendar.weeknumber ") {
                calendar_weeknumber = get_tcolor(
                    line.trim_start_matches("color.calendar.weeknumber ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.completed ") {
                completed = get_tcolor(
                    line.trim_start_matches("color.completed ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.debug ") {
                debug = get_tcolor(
                    line.trim_start_matches("color.debug ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.deleted ") {
                deleted = get_tcolor(
                    line.trim_start_matches("color.deleted ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.due ") {
                due = get_tcolor(
                    line.trim_start_matches("color.due ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.due.today ") {
                due_today = get_tcolor(
                    line.trim_start_matches("color.due.today ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.error ") {
                error = get_tcolor(
                    line.trim_start_matches("color.error ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.footnote ") {
                footnote = get_tcolor(
                    line.trim_start_matches("color.footnote ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.header ") {
                header = get_tcolor(
                    line.trim_start_matches("color.header ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.history.add ") {
                history_add = get_tcolor(
                    line.trim_start_matches("color.history.add ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.history.delete ") {
                history_delete = get_tcolor(
                    line.trim_start_matches("color.history.delete ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.history.done ") {
                history_done = get_tcolor(
                    line.trim_start_matches("color.history.done ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.label ") {
                label = get_tcolor(
                    line.trim_start_matches("color.label ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.label.sort ") {
                label_sort = get_tcolor(
                    line.trim_start_matches("color.label.sort ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.overdue ") {
                overdue = get_tcolor(
                    line.trim_start_matches("color.overdue ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.project.none ") {
                project = get_tcolor(
                    line.trim_start_matches("color.project.none ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.recurring ") {
                recurring = get_tcolor(
                    line.trim_start_matches("color.recurring ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.scheduled ") {
                scheduled = get_tcolor(
                    line.trim_start_matches("color.scheduled ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.summary.background ") {
                summary_background = get_tcolor(
                    line.trim_start_matches("color.summary.background ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.summary.bar ") {
                summary_bar = get_tcolor(
                    line.trim_start_matches("color.summary.bar ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.sync.added ") {
                sync_added = get_tcolor(
                    line.trim_start_matches("color.sync.added ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.sync.changed ") {
                sync_changed = get_tcolor(
                    line.trim_start_matches("color.sync.changed ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.sync.rejected ") {
                sync_rejected = get_tcolor(
                    line.trim_start_matches("color.sync.rejected ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.tag.next ") {
                tag_next = get_tcolor(
                    line.trim_start_matches("color.tag.next ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.tag.none ") {
                tag = get_tcolor(
                    line.trim_start_matches("color.tag.none ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.tagged ") {
                tagged = get_tcolor(
                    line.trim_start_matches("color.tagged ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.uda.priority ") {
                uda_priority = get_tcolor(
                    line.trim_start_matches("color.uda.priority ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.uda.priority.H ") {
                uda_priority_h = get_tcolor(
                    line.trim_start_matches("color.uda.priority.H ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.uda.priority.L ") {
                uda_priority_l = get_tcolor(
                    line.trim_start_matches("color.uda.priority.L ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.uda.priority.M ") {
                uda_priority_m = get_tcolor(
                    line.trim_start_matches("color.uda.priority.M ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.undo.after ") {
                undo_after = get_tcolor(
                    line.trim_start_matches("color.undo.after ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.undo.before ") {
                undo_before = get_tcolor(
                    line.trim_start_matches("color.undo.before ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.until ") {
                until = get_tcolor(
                    line.trim_start_matches("color.until ")
                        .trim_start_matches(" "),
                );
            }
            if line.starts_with("color.warning") {
                warning = get_tcolor(
                    line.trim_start_matches("color.warning ")
                        .trim_start_matches(" "),
                );
            }
        }

        Self {
            enabled,
            active,
            alternate,
            blocked,
            blocking,
            burndown_done,
            burndown_pending,
            burndown_started,
            calendar_due,
            calendar_due_today,
            calendar_holiday,
            calendar_overdue,
            calendar_today,
            calendar_weekend,
            calendar_weeknumber,
            completed,
            debug,
            deleted,
            due,
            due_today,
            error,
            footnote,
            header,
            history_add,
            history_delete,
            history_done,
            label,
            label_sort,
            overdue,
            project,
            recurring,
            scheduled,
            summary_background,
            summary_bar,
            sync_added,
            sync_changed,
            sync_rejected,
            tag_next,
            tag,
            tagged,
            uda_priority,
            uda_priority_h,
            uda_priority_l,
            uda_priority_m,
            undo_after,
            undo_before,
            until,
            warning,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::TColorConfig;
    #[test]
    fn test_colors() {
        let tc = TColorConfig::default();
        println!("{:?}", tc.due_today);
    }
}