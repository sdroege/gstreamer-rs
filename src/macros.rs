macro_rules! field_values {
    ($field_iterator: expr => $($field_value_name: literal = $field_value: expr;)*) => {
        [$((
            &$field_iterator.next().expect(stringify!("field missing for", $field_value_name)),
            crate::UnsizeValue::unsize_value(&$field_value),
        ),)*]
    };
}
