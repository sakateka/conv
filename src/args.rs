use clap::{App, Arg, ArgMatches};
use conv;
use std::ops::Deref;

pub fn build_app<'a>(name: &str) -> ArgMatches<'a> {
    App::new(name)
        .version("0.1.0")
        .author("Sergey Kacheev <uo0@ya.ru>")
        .about(format!(
                    "Корвертер текстов между кодировками {:?}",
                    conv::SUPPORTED_CODES.deref()
                ).as_ref()
        )
        .arg(Arg::with_name("from")
            .short("f")
            .long("from")
            .required(true)
            .takes_value(true)
            .help("Кодировка источника"))
        .arg(Arg::with_name("to")
            .short("t")
            .long("to")
            .required(true)
            .takes_value(true)
            .help("Кодировка результата"),
        )
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("Имя фала для записи результата")
        )
        .arg(Arg::with_name("safely")
            .short("s")
            .long("safely")
            .takes_value(false)
            .help("Прекратить конвертацию при первой ошибке")
        )
        .arg(Arg::with_name("replace")
            .short("r")
            .long("replace")
            .takes_value(true)
            .help("Символ для замены ошибок декодирования 8 битных кодировок (по умолчанию '?')")
        )
        .arg(Arg::with_name("SOURCE")
             .help("Файл для конвертации")
        ).get_matches()
}
