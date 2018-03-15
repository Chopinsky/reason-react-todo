type state = {
  items: list(TodoItem.item)
};

type action =
  | AddItem;

let component = ReasonReact.reducerComponent("TodoApp");
let stringify = ReasonReact.stringToElement;
let newItem = () => TodoItem.newItem("Click a button", true);

let make = (children) => {
  ...component,

  initialState: () => {
    items: [
      {title: "Write something to do", completed: false}
    ]
  },

  reducer: (action, {items}) => {
    switch action {
      | AddItem => ReasonReact.Update({ items: [newItem(), ...items] })
    }
  },

  render: ({state: {items}, reduce}) => {
    let itemCount = List.length(items);
    let itemCountDisplay =
      switch (itemCount) {
      | 1 => stringify("1 item")
      | _ => stringify(string_of_int(itemCount) ++ " items")
      };

    let itemsDisplay =
      switch (itemCount) {
      | 0 => stringify("Nothing yet...")
      | _ => {
          ReasonReact.arrayToElement(Array.of_list(
            List.map((item) => <TodoItem item />, items)
          ))
        }
      };

    <div className="app">
      <div className="title">
        (stringify("What to do"))
        <button onClick=(reduce((event) => AddItem))>
          (stringify("Add something"))
        </button>
      </div>
      <div className="items">
        (itemsDisplay)
      </div>
      <div className="footer">
        (itemCountDisplay)
      </div>
    </div>
  }
};
