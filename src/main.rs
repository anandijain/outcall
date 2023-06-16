use async_trait::async_trait;
use leptos::*;
use leptos_struct_table::*;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use word_generator::{langs, *};

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(sortable, classes_provider = "TableClasses")]
pub struct User {
    #[table(key)]
    id: u32,
    email: String,
    given_name: String,
    family_name: String,
}
//email

#[component]
pub fn Users(cx: Scope, users: Vec<User>) -> impl IntoView {
    // Create a few Person items
    let items = create_rw_signal(cx, users);

    // Use the generated component
    view! { cx,
        <UserTable items=items />
    }
}

fn random_users(amount: usize) -> Vec<User> {
    thread_rng().sample_iter(&Standard).take(amount).collect()
}

impl Distribution<User> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> User {
        let reader = BufReader::new(langs::FR_TXT);
        let words = generate_words(reader, 4, 2).unwrap();
        let given_name = words[1].to_owned();
        let family_name = words[0].to_owned();
        User {
            id: rng.gen::<u32>(),
            email: format!("{}.{}@acme.com", given_name, family_name),
            given_name,
            family_name,
        }
    }
}

// Easy to use with Trunk (trunkrs.dev) or with a simple wasm-bindgen setup
pub fn main() {
    let users = random_users(30);
    mount_to_body(|cx| view! { cx,  <Users users=users /> })
}

#[derive(Clone, Copy)]
pub struct TableClasses;

impl TableClassesProvider for TableClasses {
    fn new() -> Self {
        Self
    }

    fn table(&self, classes: &str) -> String {
        format!(
            "{} {}",
            "min-w-full table-fixed divide-y divide-gray-300", classes
        )
    }

    fn head_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "min-w-[12rem] py-3.5 pr-3 text-left text-sm font-semibold text-gray-900y",
            template_classes
        )
    }

    fn head_cell(&self, _sort: ColumnSort, template_classes: &str) -> String {
        format!(
            "cursor-pointer px-5 py-3 {} {}",
            "text-black", template_classes
        )
    }

    fn head_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, _row_index: usize, _selected: bool, template_classes: &str) -> String {
        format!(
            "{} {} {}",
            "border-b dark:border-gray-700",
            "whitespace-nowrap py-4 pr-3 text-sm font-medium text-gray-900",
            template_classes
        )
    }

    fn cell(&self, row_index_classes: &str) -> String {
        format!("{} {}", "px-5 py-2", row_index_classes)
    }
}
