# rustfmt removing trailing spaces

When there is a `&str` with trailing spaces rustfmt formats in a way that removes trailing spaces.

## Instructions

- Run `cargo run` to see that code runs successfully
- Run `cargo +nightly fmt --all -- --emit files`
- Run `cargo run` to see that the code fails

The strings that are generated should both have a length of 26.

## Description

**Note: Line numbers have been added and spaces are replaced with `·` for clarity.**

Assuming that `max_width = 30` is set, when trying to format a line like:

```
1| ··"457890123456789012345678··"
```
, rustfmt will suggest a new line of:

```
1| ··"457890123456789012345678·\
2| ····"
```

If you attempt to add back the space that was removed:

```
1| ··"457890123456789012345678··\
2| ····"
```

rustfmt again, suggest removing the space:

```
1| ··"457890123456789012345678·\
2| ····"
```

