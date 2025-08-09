use table::Table;

pub mod table;

pub enum Element {
    Caption,
    Col,
    Colgroup,
    Table(Table),
    Tbody,
    Td,
    Tfoot,
    Th,
    Thead,
    Tr,
}
