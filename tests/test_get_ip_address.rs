use hello_world_lib::get_ip_address;

// !!! These tests look for files in the local filesystem and must run from the project root directory.
// !!! see ./tests/integration_tests.sh.

#[actix_rt::test]
async fn test_valid_file_with_matching_hostname() {
    let dhcp_lease_file = "tests/test-data/valid_lease.txt";
    let hostname = "SMA30XXXXX5";
    let expected_ip = "192.168.100.49";

    let result = get_ip_address(dhcp_lease_file, hostname);
    assert_eq!(result, expected_ip);
}

#[actix_rt::test]
async fn test_valid_file_with_no_matching_hostname() {
    let dhcp_lease_file = "tests/test-data/valid_lease.txt";
    let hostname = "UnknownHostname";
    let expected_result = "UnknownHostname";

    let result = get_ip_address(dhcp_lease_file, hostname);
    assert_eq!(result, expected_result);
}

#[actix_rt::test]
async fn test_invalid_file_path() {
    let dhcp_lease_file = "tests/test-data/nonexistent.txt";
    let hostname = "SMA30XXXXX5";
    let expected_result = "SMA30XXXXX5";

    let result = get_ip_address(dhcp_lease_file, hostname);
    assert_eq!(result, expected_result);
}

#[actix_rt::test]
async fn test_empty_file() {
    let dhcp_lease_file = "tests/test-data/empty_lease.txt";
    let hostname = "SMA30XXXXX5";
    let expected_result = "SMA30XXXXX5";

    let result = get_ip_address(dhcp_lease_file, hostname);
    assert_eq!(result, expected_result);
}

#[actix_rt::test]
async fn test_file_with_malformed_lines() {
    let dhcp_lease_file = "tests/test-data/malformed_lease.txt";
    let hostname = "SMA30XXXXX5";
    let expected_result = "SMA30XXXXX5";

    let result = get_ip_address(dhcp_lease_file, hostname);
    assert_eq!(result, expected_result);
}
