use crate::LOGO;

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
        let mut logo_str  = vec![
                ("═════════════════════INFO════════════════════", None),
                ("📊 Всего строк", Some(self.all_count)),
                ("❌ Ошибка валидации длины", Some(self.length_error)),
                ("🔑 Ошибка валидации lp", Some(self.data_error)),
                ("⚠️ Ошибка парса строки", Some(self.parse_error)),
                ("🚫 Ошибка валидации фильтрами", Some(self.filter_error)),
                ("🔁 Ошибка одинаковые log_pass", Some(self.lp_equal)),
                ("✅ Валидных строк", Some(self.valid)),      
        ];
        if debug {
            logo_str.extend([
                ("══════════════════DEBUG MODE═════════════════", None),
                (duration.unwrap_or(String::new().leak().to_string()).leak(), None),
                (used_memory.unwrap_or(String::new().leak().to_string()).leak(), None)
            ]);
        }
        let mut logo = (*LOGO).clone();
        logo.extra_info.extend(logo_str);
        logo.render()
    }
}