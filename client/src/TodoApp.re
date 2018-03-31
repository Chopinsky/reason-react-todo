type state = {
  items: list(TodoItem.item),
  lastId: int
};

type item = TodoItem.item;

type action =
  | AddItem;

let totalCount = ref(0);
let component = ReasonReact.reducerComponent("TodoApp");
let stringify = ReasonReact.stringToElement;
let newItem = (id) => {
  totalCount := totalCount^ + 1;
  TodoItem.newItem(id, "Click a button", true)
};

let make = (children) => {
  ...component,

  initialState: () => {
    items: [
      TodoItem.newItem(0, "Write something to do", false)
    ],
    lastId: 1,
  },

  reducer: (action, {items, lastId}) => {
    switch action {
    | AddItem => {
        ReasonReact.Update({
          items: [newItem(lastId), ...items],
          lastId: lastId + 1
        });
      }
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
            List.map((item) => <TodoItem key=(string_of_int(TodoItem.getId(item))) item />, items)
          ));
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
