fn main() {
 let a = vec![
  // single-line
  "457890123456789012345678  ",
  // multi-line
  "457890123456789012345678  \
    ",
 ];
 assert_eq!(a[0].len(), 26);
 assert_eq!(a[1].len(), 26);
}
