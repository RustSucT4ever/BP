pub trait BpLoudsCommonTrait {
    fn isleaf (&self,pos:u64) -> Option<bool>;
    fn parent(&self,pos:u64) -> Option<u64>;
    fn first_child(&self,pos:u64) -> Option<u64>;
    fn next_sibling(&self,pos:u64) -> Option<u64>; 
}