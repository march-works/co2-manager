use self::carbon_deposit::CarbonDepositRepository;

pub mod carbon_deposit;

pub trait Repositories {
    type CarbonRepo: CarbonDepositRepository;

    fn carbon_repository(&self) -> &Self::CarbonRepo;
}
