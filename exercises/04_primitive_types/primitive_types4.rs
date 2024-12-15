fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // 在Rust中,切片(slice)是对数组一部分的引用
        // 1. 切片不拥有数据的所有权,只是借用了原数组的一部分
        // 2. 切片保证了内存安全 - 编译器会检查切片的生命周期
        // 3. 使用引用可以避免数据的复制,提高性能
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
