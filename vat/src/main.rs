#[derive(Debug)]
enum VatType {
    Include,
    Exclude,
}

fn create_vat(vat_type: VatType, price: f64) -> f64 {
    const VAT_RATE: f64 = 0.07;
    return match vat_type {
        VatType::Include => price * VAT_RATE / (1.0 + VAT_RATE),
        VatType::Exclude => price * VAT_RATE,
    };
}

fn main() {
    let price_with_vat = create_vat(VatType::Include, 100.0);
    println!("Price without VAT: ฿{:.2}", price_with_vat);

    let price_without_vat = create_vat(VatType::Exclude, 100.0);
    println!("Price with VAT: ฿{:.2}", price_without_vat);
}
