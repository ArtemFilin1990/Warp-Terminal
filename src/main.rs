mod agent;
mod editor;
mod integration;
mod terminal;

use agent::AgentMode;
use integration::{AgentBackend, AgentIntegration};
use terminal::{AutocompleteEngine, BlockNavigator, Highlighter, InputBuffer};

fn main() {
    println!("Warp Terminal — a fast, modern terminal built for programming with agents.\n");

    // --- Terminal UI components ---
    let mut input = InputBuffer::new();
    let highlighter = Highlighter::new();
    let autocomplete = AutocompleteEngine::new();
    let mut blocks = BlockNavigator::new();

    // --- Agent mode ---
    let mut agent = AgentMode::new();

    // --- Agent integration (voice, @-selection, backends) ---
    let mut integration = AgentIntegration::new();
    integration.connect(AgentBackend::ClaudeCode);

    // Demonstrate mode switching
    println!("Mode: {:?}", agent.mode);
    agent.toggle();
    println!("Mode after toggle: {:?}", agent.mode);

    if agent.is_dialogue() {
        agent.session.add_user("How do I list all Rust files recursively?");
        agent.session.add_agent("Run: find . -name '*.rs'");
        println!("\nAgent session ({} messages):", agent.session.messages().len());
        for msg in agent.session.messages() {
            println!("  [{:?}] {}", msg.role, msg.content);
        }
    }

    // Demonstrate syntax highlighting
    let line = "let mut x = 42; # assign";
    let tokens = highlighter.tokenize(line);
    println!("\nHighlighted tokens for '{}':", line);
    for token in &tokens {
        print!("[{:?}:'{}'] ", token.kind, token.text);
    }
    println!();

    // Demonstrate autocomplete
    let prefix = "ca";
    let completions = autocomplete.complete(prefix);
    println!("\nCompletions for '{}':", prefix);
    for c in &completions {
        println!("  {} — {}", c.value, c.description);
    }

    // Demonstrate @-selection expansion
    use integration::SelectionResource;
    integration.selections.add("main", SelectionResource::File("/src/main.rs".into()));
    let prompt = integration.build_prompt("Please review @main");
    println!("\nExpanded prompt: {}", prompt);

    // Demonstrate block navigation
    use terminal::Block;
    blocks.push(Block { command: "cargo build".into(), output: "Compiling...".into(), exit_code: 0 });
    blocks.push(Block { command: "cargo test".into(), output: "test result: ok".into(), exit_code: 0 });
    println!("\nCurrent block: {:?}", blocks.current().map(|b| &b.command));

    // Demonstrate multi-line input buffer
    input.insert_char('l');
    input.insert_char('s');
    println!("\nInput buffer: {:?}", input.text());

    println!("\nWarp Terminal initialized successfully.");
}
