#[cfg(test)]
mod tests {
    use swc_core::ecma::{transforms::testing::test_inline, visit::as_folder};
    use super::super::visitor::TransformVisitor;
    use swc_core::ecma::parser::{Syntax, TsSyntax};

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        basic_function_name,
        r#"function GoodButton() {
            return <Button>Click me</Button>;
        }"#,
        r#"function GoodButton() {
            return <Button data-testid="GoodButton">Click me</Button>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        nested_components,
        r#"function CardContainer() {
            return (
                <Card>
                    <Button>Click me</Button>
                </Card>
            );
        }"#,
        r#"function CardContainer() {
            return <Card data-testid="CardContainer">
                    <Button>Click me</Button>
                </Card>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        complex_nested_structure,
        r#"function DashboardLayout() {
            return (
                <Layout>
                    <Sidebar>
                        <Navigation>
                            <MenuItem>Home</MenuItem>
                        </Navigation>
                    </Sidebar>
                    <Content>
                        <Header>Dashboard</Header>
                        <Main>Content</Main>
                    </Content>
                </Layout>
            );
        }"#,
        r#"function DashboardLayout() {
            return <Layout data-testid="DashboardLayout">
                    <Sidebar>
                        <Navigation>
                            <MenuItem>Home</MenuItem>
                        </Navigation>
                    </Sidebar>
                    <Content>
                        <Header>Dashboard</Header>
                        <Main>Content</Main>
                    </Content>
                </Layout>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        mixed_html_and_components,
        r#"function UserProfile() {
            return (
                <div className="container">
                    <ProfileCard>
                        <img src="avatar.jpg" />
                        <UserInfo>
                            <h1>John Doe</h1>
                            <span>Developer</span>
                        </UserInfo>
                    </ProfileCard>
                </div>
            );
        }"#,
        r#"function UserProfile() {
            return <div className="container" data-testid="UserProfile">
                    <ProfileCard>
                        <img src="avatar.jpg" />
                        <UserInfo>
                            <h1>John Doe</h1>
                            <span>Developer</span>
                        </UserInfo>
                    </ProfileCard>
                </div>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        multiple_root_components,
        r#"function TabPanel() {
            return (
                <>
                    <Tab>First</Tab>
                    <Tab>Second</Tab>
                    <Panel>
                        <Content>Tab content</Content>
                    </Panel>
                </>
            );
        }"#,
        r#"function TabPanel() {
            return <>
                    <Tab data-testid="TabPanel">First</Tab>
                    <Tab>Second</Tab>
                    <Panel>
                        <Content>Tab content</Content>
                    </Panel>
                </>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        preserve_existing_custom_attr,
        r#"function CustomCard() {
            return <Card data-testid="my-special-card">
                    <Button>Click me</Button>
                </Card>;
        }"#,
        r#"function CustomCard() {
            return <Card data-testid="my-special-card">
                    <Button>Click me</Button>
                </Card>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        with_conditional_rendering,
        r#"function ConditionalComponent({ isVisible }) {
            return (
                <Container>
                    {isVisible ? (
                        <Alert>Visible</Alert>
                    ) : (
                        <Message>Hidden</Message>
                    )}
                </Container>
            );
        }"#,
        r#"function ConditionalComponent({ isVisible }) {
            return <Container data-testid="ConditionalComponent">
                    {isVisible ? <Alert>Visible</Alert> : <Message>Hidden</Message>}
                </Container>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-test-id".to_string()))),
        different_attribute_name,
        r#"function TestComponent() {
            return <Card>Test</Card>;
        }"#,
        r#"function TestComponent() {
            return <Card data-test-id="TestComponent">Test</Card>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        arrow_function_component,
        r#"const Sidebar = () => {
            return (
                <Card>
                    <Button>Click me</Button>
                </Card>
            );
        }"#,
        r#"const Sidebar = () => {
            return <Card data-testid="Sidebar">
                    <Button>Click me</Button>
                </Card>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        arrow_function_with_props,
        r#"const UserCard = ({ name, role }: { name: string; role: string }) => {
            return (
                <Card>
                    <Avatar />
                    <UserInfo>
                        <Name>{name}</Name>
                        <Role>{role}</Role>
                    </UserInfo>
                </Card>
            );
        }"#,
        r#"const UserCard = ({ name, role }: { name: string; role: string }) => {
            return <Card data-testid="UserCard">
                    <Avatar />
                    <UserInfo>
                        <Name>{name}</Name>
                        <Role>{role}</Role>
                    </UserInfo>
                </Card>;
        }"#
    );

    test_inline!(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        |_| as_folder(TransformVisitor::new(Some("data-testid".to_string()))),
        mixed_function_types,
        r#"function OuterComponent() {
            const InnerComponent = () => {
                return <Button>Click</Button>;
            };
            return (
                <Card>
                    <InnerComponent />
                </Card>
            );
        }"#,
        r#"function OuterComponent() {
            const InnerComponent = () => {
                return <Button data-testid="InnerComponent">Click</Button>;
            };
            return <Card data-testid="OuterComponent">
                    <InnerComponent />
                </Card>;
        }"#
    );

    #[test]
    fn test_visitor_default_attr() {
        let visitor = TransformVisitor::new(None);
        assert_eq!(visitor.get_attribute_name(), "data-test-id");
    }

    #[test]
    fn test_visitor_custom_attr() {
        let visitor = TransformVisitor::new(Some("data-testid".to_string()));
        assert_eq!(visitor.get_attribute_name(), "data-testid");
    }
} 