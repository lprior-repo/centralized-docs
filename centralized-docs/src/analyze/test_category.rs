use super::category::detect_category;

#[test]
fn test_detect_category_readme() {
    assert_eq!(detect_category("README.md", "some content"), "meta");
}

#[test]
fn test_detect_category_changelog() {
    assert_eq!(detect_category("CHANGELOG.md", "content"), "meta");
}

#[test]
fn test_detect_category_contributing() {
    assert_eq!(detect_category("CONTRIBUTING.md", "content"), "meta");
}

#[test]
fn test_detect_category_license() {
    assert_eq!(detect_category("LICENSE", "content"), "meta");
}

#[test]
fn test_detect_category_security() {
    assert_eq!(detect_category("SECURITY.md", "content"), "meta");
}

#[test]
fn test_detect_category_code_of_conduct() {
    assert_eq!(detect_category("CODE_OF_CONDUCT.md", "content"), "meta");
}

#[test]
fn test_detect_category_index_file() {
    assert_eq!(detect_category("INDEX.md", "content"), "meta");
}

#[test]
fn test_detect_category_tutorial_content() {
    assert_eq!(
        detect_category("guide.md", "This is a tutorial on testing"),
        "tutorial"
    );
}

#[test]
fn test_detect_category_getting_started() {
    assert_eq!(
        detect_category("start.md", "Getting started with our tool"),
        "tutorial"
    );
}

#[test]
fn test_detect_category_quickstart_content() {
    assert_eq!(
        detect_category("intro.md", "Follow this quickstart guide"),
        "tutorial"
    );
}

#[test]
fn test_detect_category_quickstart_filename() {
    assert_eq!(
        detect_category("quickstart.md", "random content"),
        "tutorial"
    );
}

#[test]
fn test_detect_category_tutorial_filename() {
    assert_eq!(detect_category("tutorial.md", "random content"), "tutorial");
}

#[test]
fn test_detect_category_ref_content() {
    assert_eq!(
        detect_category("docs.md", "The api provides HTTP endpoints"),
        "ref"
    );
}

#[test]
fn test_detect_category_reference_content() {
    assert_eq!(
        detect_category("info.md", "See the reference documentation"),
        "ref"
    );
}

#[test]
fn test_detect_category_function_content() {
    assert_eq!(
        detect_category("lib.md", "The function main() does things"),
        "ref"
    );
}

#[test]
fn test_detect_category_class_content() {
    assert_eq!(
        detect_category("oop.md", "The class Animal has methods"),
        "ref"
    );
}

#[test]
fn test_detect_category_api_filename() {
    assert_eq!(detect_category("api.md", "random content"), "ref");
}

#[test]
fn test_detect_category_reference_filename() {
    assert_eq!(detect_category("reference.md", "random content"), "ref");
}

#[test]
fn test_detect_category_ops_content() {
    assert_eq!(
        detect_category("deploy.md", "This is a how-to guide for deployment"),
        "ops"
    );
}

#[test]
fn test_detect_category_how_to_content() {
    assert_eq!(
        detect_category("steps.md", "how to configure the system"),
        "ops"
    );
}

#[test]
fn test_detect_category_guide_content() {
    assert_eq!(
        detect_category("setup.md", "Follow this guide to install"),
        "ops"
    );
}

#[test]
fn test_detect_category_how_to_filename() {
    assert_eq!(detect_category("how-to-deploy.md", "random"), "ops");
}

#[test]
fn test_detect_category_guide_filename() {
    assert_eq!(detect_category("guide.md", "random"), "ops");
}

#[test]
fn test_detect_category_deployment_filename() {
    assert_eq!(detect_category("deployment.md", "random"), "ops");
}

#[test]
fn test_detect_category_fallback_concept() {
    assert_eq!(
        detect_category("random-file.md", "Just some random content about things."),
        "concept"
    );
}

#[test]
fn test_detect_category_meta_beats_tutorial() {
    assert_eq!(
        detect_category("readme.md", "This is a tutorial getting started guide"),
        "meta"
    );
}

#[test]
fn test_detect_category_tutorial_beats_ref() {
    assert_eq!(
        detect_category("guide.md", "api reference tutorial"),
        "tutorial"
    );
}

#[test]
fn test_detect_category_case_insensitive_filename() {
    assert_eq!(detect_category("README.MD", "content"), "meta");
    assert_eq!(detect_category("Readme.md", "content"), "meta");
}

#[test]
fn test_detect_category_with_frontmatter() {
    let content = "---\ntitle: My Tutorial\n---\n\nThis is getting started content.";
    assert_eq!(detect_category("guide.md", content), "tutorial");
}
