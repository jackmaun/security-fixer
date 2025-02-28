mod fetch_vulnerabilities;
mod ai_fix;
mod edit_code;
mod handle_git;
mod train_ai;

use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut final_output: Value = json!({
        "paging": {
            "pageIndex": 1,
            "pageSize": 100,
            "total": 0
        },
        "hotspots": []
    });

    let mut hotspot_list: Vec<Value> = Vec::new();

    match fetch_vulnerabilities::fetch_vulnerabilities() {
        Ok(issues) => {
            for (index, issue) in issues.iter().enumerate() {
                println!(
                    "[!] Found Issue in {} at line {}: {}",
                    issue.component, issue.line, issue.message
                );

                let example_code = "String password = '12345';"; // just a placeholder until real code extraction

                match ai_fix::generate_fix(&issue.message, example_code) {
                    Ok((fix, explanation)) => {
                        println!("[+] Suggested Fix:\n{}", fix);
                        println!("[+] AI Explanation: {}", explanation);

                        if let Err(e) = edit_code::apply_fix(&issue.component, issue.line, &fix) {
                            eprintln!("[-] Error applying fix: {}", e);
                        }
                        else {
                            println!("[+] Fix applied successfully!");

                            if let Err(e) = train_ai::train_ai(&fix, example_code, &issue.message) {
                                eprintln!("[-] AI Training Error: {}", e);
                            }

                            let hotspot_entry = json!({
                                "key": format!("issue-{}", index + 1),
                                "component": issue.component,
                                "project": "WebGoat",
                                "securityCategory": "auth",
                                "vulnerabilityProbability": "HIGH",
                                "status": "TO_REVIEW",
                                "line": issue.line,
                                "message": issue.message,
                                "author": "unclechat@auto.fix",
                                "creationDate": "2025-02-26T00:00:00-0700",
                                "updateDate": "2025-02-26T00:00:00-0700",
                                "textRange": {
                                    "startLine": issue.line,
                                    "endLine": issue.line,
                                    "startOffset": 10,
                                    "endOffset": 50
                                },
                                "flows": [],
                                "ruleKey": "java:S6418",
                                "messageFormattings": []
                            });

                            hotspot_list.push(hotspot_entry);

                            if let Err(e) = handle_git::commit_and_push() {
                                eprintln!("[-] Git Error: {}", e);
                            }
                            else {
                                println!("[+] Fix committed and pushed.");
                            }
                        }
                    }
                    Err(e) => eprintln!("[-] AI Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("[-] Error fetching vulnerabilities: {}", e),
    }

    final_output["paging"]["total"] = Value::from(hotspot_list.len());
    final_output["hotspots"] = Value::from(hotspot_list);

    let output_file_path = "security_hotspots_output.json";
    let mut file = File::create(output_file_path).expect("[-] Failed to create JSON output file");
    file.write_all(final_output.to_string().as_bytes())
        .expect("[-] Failed to write JSON output");

    println!("\n[+] Security Hotspots Saved to `{}`", output_file_path);
}
