#![feature(coroutines)]
extern crate alloc;
#[must_use]
pub struct PartialBlockPartialTemplate0<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
#[must_use]
pub struct PartialBlockPartialTemplate2<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
impl<PartialType, EndType> PartialBlockPartialTemplate2<PartialType, EndType> {
    pub fn map_inner<NewPartialType, NewEndType>(
        self,
        new_partial_type: NewPartialType,
        new_end_type: NewEndType,
    ) -> PartialBlockPartialTemplate2<NewPartialType, NewEndType> {
        PartialBlockPartialTemplate2 {
            partial_type: new_partial_type,
            end_type: new_end_type,
        }
    }
}
// every type could probably have a non-generic companion struct and them that is an associate type with the actual struct?
pub trait Templaty<Partial, End> {

}
impl<PartialTypePartial, PartialTypeEnd, PartialType: Templaty<PartialTypePartial, PartialTypeEnd>, EndType> PartialBlockPartialTemplate0<PartialType, EndType> {
    pub fn partial_block_partial_template0(
        template: Self,
    ) -> PartialType<(), PartialBlockPartialTemplate2<(), EndType>> {
        todo!()
    }
}

pub fn main() {}