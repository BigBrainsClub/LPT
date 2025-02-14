use crate::logo::{logo, get_logo};

#[derive(Default)]
pub struct Counters {
    pub all_count: usize,
    pub length_error: usize,
    pub data_error: usize,
    pub parse_error: usize,
    pub filter_error: usize,
    pub valid: usize,
    pub lp_equal: usize
}

impl Counters {
    pub fn format_multi_line(&self, duration: Option<String>, used_memory: Option<String>, debug: bool) -> String {
        let logo_str = get_logo(
                "    ═════════════════════INFO════════════════════",
                &format!("    📊 Всего строк:                {}", self.all_count),
                &format!("    ❌ Ошибка валидации длины:     {}", self.length_error),
                &format!("    🔑 Ошибка валидации lp:        {}", self.data_error),
                &format!("    ⚠️ Ошибка парса строки:        {}", self.parse_error),
                &format!("    🚫 Ошибка валидации фильтрами: {}", self.filter_error),
                &format!("    🔁 Ошибка одинаковые log_pass: {}", self.lp_equal),
                &format!("    ✅ Валидных строк:             {}", self.valid),
                if debug {"    ══════════════════DEBUG MODE═════════════════"} else {""},
                &format!("    {}", duration.unwrap_or(String::new())),
                &format!("    {}", used_memory.unwrap_or(String::new()))
            );
        logo(&logo_str)
    }
}