// 特定のケースに特化させる処理の定義
trait Supplier {
    fn send(&self) -> String;
}

// 特定のケースの生成だけをほかに委ねる処理クラス
trait Processor {
    type SupplierImpl: Supplier;

    fn get_supplier(&self) -> Self::SupplierImpl;

    fn run(&self) -> String {
        let supplier = self.get_supplier();
        format!("process start, {}, process end", supplier.send())
    }
}

// 特定のケースに特化した処理の実装
#[derive(Clone)]
struct SagawaSupplier {
    name: String,
}

impl Supplier for SagawaSupplier {
    fn send(&self) -> String {
        format!("{} sent", self.name)
    }
}

// 特定のケースの生成の実装
struct SagawaProcessor {
    supplier: SagawaSupplier,
}

impl SagawaProcessor {
    fn new() -> Self {
        Self {
            supplier: SagawaSupplier { name: "sagawa".to_string() }
        }
    }
}

impl Processor for SagawaProcessor {
    type SupplierImpl = SagawaSupplier;

    fn get_supplier(&self) -> Self::SupplierImpl {
        self.supplier.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::creational_patterns::factory_method::{Processor, SagawaProcessor};

    #[test]
    fn case1() {
        let sut = SagawaProcessor::new();
        assert_eq!(sut.run(), "process start, sagawa sent, process end");
    }

}
