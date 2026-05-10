// // use lightningcss::values::calc::Calc
// use crate::{Value, settings::MapType, values::Number};

// impl From<lightningcss::stylesheet::StyleSheet<'_, '_>> for Value {
//     fn from(sheet: lightningcss::stylesheet::StyleSheet) -> Self {
//         let mut rules_out = Vec::new();

//         for rule in sheet.rules.0 {
//             rules_out.push(Value::from(rule));
//         }

//         Value::Vec(rules_out)
//     }
// }

// impl From<lightningcss::rules::CssRule<'_>> for Value {
//     fn from(rule: lightningcss::rules::CssRule) -> Self {
//         match rule {
//             lightningcss::rules::CssRule::Style(style) => {
//                 let mut map = MapType::new();

//                 // selectors
//                 map.insert(
//                     Value::String("selectors".into()),
//                     Value::String(style.selectors.to_string()),
//                 );

//                 // declarations
//                 let mut decls = MapType::new();
//                 for decl in style.declarations.declarations {
//                     decls.insert(
//                         Value::String(decl.property_id().name().to_string()),
//                         Value::from(decl.value),
//                     );
//                 }

//                 map.insert(Value::String("declarations".into()), Value::Map(decls));
//                 Value::Map(map)
//             }

//             _ => {
//                 // fallback for unsupported rule types
//                 Value::String(format!("{:?}", rule))
//             }
//         }
//     }
// }

// impl From<lightningcss::properties::Property<'_>> for Value {
//     fn from(prop: lightningcss::properties::Property) -> Self {
//         // safest universal fallback
//         Value::String(prop.to_css_string(Default::default()).unwrap_or_default())
//     }
// }