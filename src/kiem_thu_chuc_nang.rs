use std::{ collections::HashMap };

type UserID = String;
type BookID = i32;

#[derive(Debug, Clone)]
struct Book {
  id: BookID,
  title: String,
  author: String,
  quantity: i32,
}

#[derive(Debug, Clone)]
struct Person {
  id: UserID,
  name: String,
}

#[derive(Debug, Clone)]
struct BookLibrary {
  books: Vec<Book>,
  renting_list: HashMap<BookID, HashMap<UserID, i32>>,
}

#[derive(Debug, PartialEq)]
enum BookRentStatus {
  NotFound,
  InvalidBookCount
}

impl BookLibrary {
  pub fn new() -> Self {
    BookLibrary {
      books: Vec::<Book>::new(),
      renting_list: HashMap::<BookID, HashMap<UserID, i32>>::new(),
    }
  }

  pub fn add_book(&mut self, title: &str, author: &str, quantity: i32) {
    let ref mut list_book = self.books;
    let total_book = list_book.len() as i32;
    let new_book = Book {
      id: total_book + 1,
      author: author.to_string(),
      quantity,
      title: title.to_string(),
    };

    list_book.push(new_book);
  }

  pub fn rent_book(
    &mut self,
    person: &Person,
    book_id: i32,
    _total: Option<i32>
  ) -> Result<bool, BookRentStatus> {
    let total = _total.unwrap_or(1);

    if total < 0 {
      return Err(BookRentStatus::InvalidBookCount);
    }
    

    let book = match
      self.books
        .clone()
        .into_iter()
        .find(|b| { b.id == book_id })
    {
      Some(v) => v,
      None => return Err(BookRentStatus::NotFound)
    };

    match self.renting_list.get_mut(&book_id) {
      Some(renting) => {
        let total_renting = renting
          .clone()
          .values()
          .fold(0i32, |c, v| c + v);
        if total > book.quantity - total_renting {
          return Ok(false);
        }

        match renting.get_mut(&person.id) {
          Some(r) => {
            *r += total;
          }
          None => {
            renting.insert(person.id.clone(), total);
          }
        };
      }
      None => {
        if total > book.quantity {
          return Ok(false);
        }

        self.renting_list.insert(book_id, BookLibrary::create_new_rent(person.id.clone(), total));
      }
    }

    Ok(true)
  }

  fn total_renting_of_book(&self, book_id: i32) -> i32 {
    match self.renting_list.get(&book_id) {
      Some(r) => r.values().fold(0i32, |c, v| c + v),
      None => 0,
    }
  }

  fn person_renting_count(&self, user_id: UserID) -> i32 {
    self.renting_list
      .values()
      .fold(0i32, |total, rent_data| total + *rent_data.get(&user_id).unwrap_or(&0i32))
  }

  fn person_renting_book_count(user_id: UserID) {}

  fn create_new_rent(user_id: UserID, total: i32) -> HashMap<UserID, i32> {
    HashMap::from([(user_id, total)])
  }
}

fn main() {}

#[cfg(test)]
mod tests {
  use std::borrow::Borrow;

  use super::*;

  fn init() -> BookLibrary {
    let mut list_library = BookLibrary::new();
    list_library.add_book("Sách 1", "Nguyễn A", 2);
    list_library.add_book("Sách 2", "Nguyễn B", 3);
    list_library.add_book("Sách 3", "Nguyễn C", 4);
    list_library.add_book("Sách 4", "Nguyễn D", 5);

    list_library
  }

  fn init_with_rented(person: &Person, total_rent: i32) -> BookLibrary {
    let mut list_library = BookLibrary::new();
    list_library.add_book("Sách 1", "Nguyễn A", 3);
    list_library.add_book("Sách 2", "Nguyễn B", 3);
    list_library.add_book("Sách 3", "Nguyễn C", 3);
    list_library.add_book("Sách 4", "Nguyễn D", 3);

    list_library.rent_book(person, 2, Some(total_rent));

    list_library
  }

  #[test]
  fn rent_success_1() {
    let mut list_library = init();

    let person = Person {
      id: "20020389".to_string(),
      name: "Nguyen Dai".to_string(),
    };

    let rented_book = list_library.rent_book(person.borrow(), 2, Some(3)).unwrap();

    assert_eq!(rented_book, true)
  }

  #[test]
  fn rent_over_total_book_1() {
    let mut list_library = init();

    let person = Person {
      id: "20020389".to_string(),
      name: "Nguyen Dai".to_string(),
    };

    let rented_book = list_library.rent_book(person.borrow(), 2, Some(5)).unwrap();

    assert_eq!(rented_book, false)
  }

  #[test]
  fn rent_success_2() {
    let person = Person {
      id: "20020389".to_string(),
      name: "Nguyen Dai".to_string(),
    };

    let mut list_library = init_with_rented(person.borrow(), 2);

    let rented_book = list_library.rent_book(person.borrow(), 2, Some(1)).unwrap();

    assert_eq!(rented_book, true)
  }

  #[test]
  fn rent_over_total_book_2() {
    let person = Person {
      id: "20020389".to_string(),
      name: "Nguyen Dai".to_string(),
    };

    let mut list_library = init_with_rented(person.borrow(), 2);

    let rented_book = list_library.rent_book(person.borrow(), 2, Some(3)).unwrap();

    assert_eq!(rented_book, false)
  }

  #[test]
  fn rent_book_not_exist() {
    let mut list_library = init();

    let person = Person {
      id: "20020389".to_string(),
      name: "Nguyen Dai".to_string(),
    };

    let err = list_library.rent_book(person.borrow(), 5, Some(3)).unwrap_err();

    assert_eq!(err, BookRentStatus::NotFound)
  }

  #[test]
  fn rent_book_count_invalid() {
    let mut list_library = init();

    let person = Person {
      id: "20020389".to_string(),
      name: "Nguyen Dai".to_string(),
    };

    let err = list_library.rent_book(person.borrow(), 5, Some(-1)).unwrap_err();

    assert_eq!(err, BookRentStatus::InvalidBookCount)
  }
}
