This crate provides `Base58id`, a short, human-readable identifier for
infrequently generated records.

* **Short.** 11 characters long.
* **Human-Readable.** No similar looking characters like 0 (zero) and O (capital o).
* **URL-Friendly.** Alphanumeric characters only.
* **Efficient.** Nicely fits into `u64`.

It looks like `3mJr7AoUXx2`.