use sigmaos_svntogit_packages::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_svntogit_packages::SvntogitPackages::new("sigmaos-svntogit-packages").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-svntogit-packages"));
}
