// use std::ops::Deref;
//
// use crate::{LuaType, LuaUnionType};
//
// pub fn and_type(left_type: LuaType, right_type: LuaType) -> LuaType {
//     match (&left_type, &right_type) {
//         (LuaType::Any | LuaType::Unknown, _) => return right_type,
//         (_, LuaType::Any | LuaType::Unknown) => return left_type,
//         // union
//         (LuaType::Union(left), right) if !right.is_union() => {
//             let left = left.deref().clone();
//             if left.get_types().iter().any(|it| it == right) {
//                 return right_type;
//             }
//         }
//         (left, LuaType::Union(right)) if !left.is_union() => {
//             let right = right.deref().clone();
//             if right.get_types().iter().any(|it| it == left) {
//                 return left_type;
//             }
//         }
//         // two union
//         (LuaType::Union(left), LuaType::Union(right)) => {
//             let left = left.deref().clone();
//             let right = right.deref().clone();
//             let left_types = left.get_types();
//             let right_types = right.get_types();
//             let mut types = left_types
//                 .iter()
//                 .filter(|it| right_types.iter().any(|t| it == &t))
//                 .map(|it| it.clone())
//                 .collect::<Vec<_>>();
//             types.dedup();
//
//             if types.is_empty() {
//                 return LuaType::Nil;
//             } else if types.len() == 1 {
//                 return types[0].clone();
//             } else {
//                 return LuaType::Union(LuaUnionType::new(types).into());
//             }
//         }
//
//         // same type
//         (left, right) if left == right => return left_type.clone(),
//         _ => {}
//     }
//
//     // or maybe never
//     LuaType::Nil
// }
