pub mod ticket {
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ticket::Ticket;

    #[allow(dead_code, unused_variables)]
    fn should_not_be_possible() {
        let ticket: Ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // `description` is private, so external modules can't access it directly.
        // assert_eq!(ticket.description, "A description");
    }

    #[allow(dead_code)]
    fn encapsulation_cannot_be_violated() {
        // Fields are private, so `Ticket::new` is the only way to create a ticket
        // from outside the `ticket` module.
        /*
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
        */
    }
}
