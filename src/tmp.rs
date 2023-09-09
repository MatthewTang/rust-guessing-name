fn truthy() -> bool {
    return true;
}

#[cfg(test)]
mod tests {
    use super::truthy;

    #[test]
    fn test_truthy() {
        assert_eq!(truthy(), true);
    }
}
