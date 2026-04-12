Чек по операции
15 января 2036 13:00:00 мск
Мобильное приложение банк
Онлайн
Плательщик
БЕТОН ДЕРЕВО
ВОДА
Способ оплаты
Платёжный счёт •• 5555
Мобильная связь
Услуга
Номер телефона
+7 (111) 777-99-99
Доп. информация
Оплатили не тот номер или хотите
узнать статус платежа? Звоните
33333 или bee.ru.
Идентификатор платежа (СУИП)
565666265262AAAA
Получатель
ПАО "ВымелКом"
ИНН
2222222222
КПП
366666666
БИК
200000000
10000000000000000000
Расчётный счёт
Корр. счёт
00000000000000000000
Тип операции
Безналичная оплата услуг
Код авторизации
000000
Сумма платежа
700,00 ₽
Комиссия
0,00 ₽
Итого
700,00 ₽
Семьсот рублей 00 копеек



// Функция для поиска и обрезки данных
fn extract(source, label, end_label) {
    let start_idx = source.index_of(label);
    if start_idx < 0 { return ""; }

    let start_pos = start_idx + label.len();
    let sub = "";

    if end_label != "" {
        let end_idx = source.index_of(end_label);
        if end_idx > start_pos {
            sub = source.sub_string(start_pos, end_idx - start_pos);
        } else {
            sub = source.sub_string(start_pos);
        }
    } else {
        sub = source.sub_string(start_pos);
    }
    sub.trim();
    return sub;
}

fn process(text){

// 1. Извлекаем ФИО
// Вместо replace используем логику разделения и соединения, если replace недоступен
let fio_raw = extract(text, "Плательщик", "Способ оплаты");
let fio_parts = fio_raw.split("\n");
let fio = "";
for p in fio_parts {
    p.trim();
    if p.len > 0 {
        if fio.len() > 0 { fio += " "; }
        fio += p;
    }
}

// 2. Номер телефона
let phone = extract(text, "Номер телефона", "Доп. информация");

// 3. БИК
let bik = extract(text, "БИК", "Расчётный счёт");
// Если после БИК идет сразу счет на той же строке, берем первые 9 цифр
if bik.len > 9 {
    bik = bik.sub_string(0, 9);
}

// 4. Сумма
let amount_full = extract(text, "Сумма платежа", "Комиссия");
// Удаляем символ валюты через подстроку, если replace не работает
let amount = amount_full;
let currency_idx = amount.index_of(" ₽");
if currency_idx >= 0 {
    amount = amount.sub_string(0, currency_idx);
}

let result = #{
    fio: fio,
    phone_number: phone,
    bik: bik,
    amount: amount
};

return result;
}


//======================
fn get_value(source, label) {
    let start_idx = source.index_of(label);
    if start_idx < 0 { return ""; }

    // Переходим к концу метки и убираем лишние пробелы/переносы
    let value_part = source.sub_string(start_idx + label.len);
    let lines = value_part.split("\n");

    // Берем первую непустую строку после метки
    for line in lines {
        line.trim();
        if line.len > 0 {
            return line;
        }
    }
    return "";
}

fn process(text){
let start_fio = text.index_of("Плательщик") + "Плательщик".len;
let end_fio = text.index_of("Способ оплаты");
let fio_raw = text.sub_string(start_fio, end_fio - start_fio);
fio_raw.trim();
fio_raw.replace("\n", " ");

// 2. Извлекаем Номер телефона
let phone = get_value(text, "Номер телефона");

// 3. Извлекаем БИК
let bik = get_value(text, "БИК");

// 4. Извлекаем Сумму платежа
let amount_raw = get_value(text, "Сумма платежа");
amount_raw.replace(" ₽", ""); // Убираем символ рубля

// Формируем итоговый объект
let result = #{
    fio: fio_raw,
    phone_number: phone,
    bik: bik,
    amount: amount_raw
};
return result;
}
