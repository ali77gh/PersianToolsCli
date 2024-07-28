use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// if you don't pass --input param it will take std-in as input
    #[arg(short, long)]
    pub input: Option<String>,

    /// name of function see function list by running --help
    #[command(subcommand)]
    pub function: Function,
}

#[derive(Debug, Subcommand)]
pub enum Function {
    /// "۵۴۱" -> "541"
    DigitsFaToEn,

    /// "541" -> "۵۴۱"
    DigitsEnToFa,

    /// "5677" -> "پنج هزار و ششصد و هفتاد و هفت"
    NumberToWords,

    /// حروف فارسی رو به عدد تبدیل میکنه
    WordsToNumber,

    /// Returns list of card numbers extracted from input separated with ','
    ExtractCardNumber,

    /// 6219861000000000 -> "بانک سامان"
    GetBankNameByCardNumber,

    /// شماره کارت بانکی رو اعتبار سنجی میکنه
    VerifyCardNumber,

    /// شماره شبا رو اعتبار سنجی میکنه
    IsShebaValid,

    /// شماره شبا رو میگیره و اسم بانک بر میگردونه
    ShebaToBankName,

    /// شماره شبا رو میگیره و اسم فارسی بانک بر میگردونه
    ShebaToPersianBankName,

    /// <national_id> -> "کرج"
    GetCityByIranNationalId,

    /// <national_id> -> "البرز"
    GetProvinceByIranNationalId,

    /// اعتبار سنجی کد ملی
    /// Returns true|false
    VerifyIranianNationalId,

    /// Input is a car or motorcycle number plate
    /// Returns (car | motorcycle)
    GetPlateType,

    /// Input is a car or motorcycle number plate
    /// Returns نام استان
    GetPlateProvince,

    /// Input is a car or motorcycle number plate
    /// Returns (دیپلمات - سفارتخانه - تاکسی - ارتش - شخصی و...)
    GetPlateCategory,

    /// Phone number as input and returns (false|true)
    IsPhoneValid,

    /// 09387891234 -> 938,
    /// +989387891234 -> 938,
    /// 00989387891234 -> 938
    GetOperatorPrefix,

    /// 09380000000 -> Irancell
    GetPhoneOperator,

    /// 09140000000 -> تبریز
    GetPhoneProvince,

    /// Adds commas to number,
    ///
    /// example: 3100 -> 3,100
    AddCommas,

    /// Description: Replaces all instances of ي and ك with ی and ک,
    ToPersianChars,

    /// Return true if the entered string includes persian characters
    HasPersian,

    /// Return true if the entered string does not include other-language characters.
    IsPersian,

    /// اعتبار سنجی شناسه حقوقی
    /// Returns true|false
    VerifyIranianLegalId,

    /// "البرز" -< "کرج"
    FindCapitalByProvince,

    /// Gets barcode as input and returns bill type
    /// types: (Water, Electricity, Gas, Tel, Mobile, Municipality, Tax, DrivingOffense)
    GetBillType,

    /// Gets barcode as input and returns bill amount in Rials
    GetBillAmount,

    /// حروف فارسی رو به فرمتی تبدیل میکنه که در url قابل استفاده باشه
    UrlFix,

    /// Opposite of add-half-space
    RemoveHalfSpace,

    /// Takes input and make it standard in case of using half space
    AddHalfSpace,

    /// Description: Replaces all instances of ی and ک with  ي and ك,
    ToArabic,

    /// Return true if the entered string includes arabic characters
    HasArabic,

    /// Return true if the entered string does not include other-language characters.
    IsArabic,

    /// "451" -> "٤٥۱"
    DigitsEnToAr,

    /// "٤٥۱" -> "451"
    DigitsArToEn,

    /// "۴۵۱" -> "٤٥۱"
    DigitsFaToAr,

    /// "٤٥۱" -> "451"
    DigitsArToFa,

    /// Remove commas from number,
    /// example: 3,100 -> 3100
    RemoveCommas,

    /// Add persian ordinal suffix to numbers,
    /// example: "بیست و یک" -< "بیست و یکم"
    AddOrdinalSuffix,

    /// Remove persian ordinal suffix from numbers,
    /// example: "بیست و یکم" -< "بیست و یک"
    RemoveOrdinalSuffix,

    ///برای دو لحظه از زمان یک متن فارسی تولید میکنه که اختلاف دو لحظه رو توصیف میکنه
    TimeDiff,
}
