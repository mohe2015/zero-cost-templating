pub struct PartialBlockPartialTemplate0<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
pub struct PartialBlockPartialTemplate2<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
// every type could probably have a non-generic companion struct and them that is an associate type with the actual struct?
pub trait Templaty<Partial, End> {

}
// we should try this generically because then templates could be used cross-crate
impl<PartialTypePartial, PartialTypeEnd, PartialType: Templaty<PartialTypePartial, PartialTypeEnd>, EndType> PartialBlockPartialTemplate0<PartialType, EndType> {
    pub fn partial_block_partial_template0(
        template: Self,
    ) -> PartialType<(), PartialBlockPartialTemplate2<(), EndType>> {
        todo!()
    }
}

pub fn main() {}