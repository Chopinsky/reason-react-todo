type state = {
  items: list(TodoItem.item),
  lastId: int
};

let valueFromEvent = (evt): string => (
  evt
  |> ReactEventRe.Form.target
  |> ReactDOMRe.domElementToObj
)##value;

module Input = {
  type state = string;
  let component = ReasonReact.reducerComponent("Input");

  let make = (~onSubmit, _) => {
    ...component,

    initialState: () => "",

    reducer: (newText, _text) => ReasonReact.Update(newText),

    render: ({ state: text, reduce }) =>
      <input
        value=text
        _type="text"
        placeholder="What to do next..."
        onChange=(reduce((evt) => valueFromEvent(evt)))
        onKeyDown=((evt) =>
          if (ReactEventRe.Keyboard.key(evt) == "Enter") {
            onSubmit(text);
            (reduce(() => ""))()
          }
        )
      />
  }
};

type item = TodoItem.item;
type action =
  | AddItem(string)
  | ToggleItem(int);

let totalCount = ref(0);
let component = ReasonReact.reducerComponent("TodoApp");
let stringify = ReasonReact.stringToElement;
let newItem = (id, text) => {
  totalCount := totalCount^ + 1;
  TodoItem.newItem(id, text, true)
};

let make = (_) => {
  ...component,

  initialState: () => {
    items: [
      TodoItem.newItem(0, "Write something to do", false)
    ],
    lastId: 1,
  },

  reducer: (action, {items, lastId}) => {
    switch action {
    | AddItem(text) => {
        ReasonReact.Update({
          items: [newItem(lastId, text), ...items],
          lastId: lastId + 1
        });
      }
    | ToggleItem(id) => {
        let items = List.map((itm) =>
          TodoItem.getId(itm) === id ? {...itm, completed: !itm.completed} : itm
          , items);
        ReasonReact.Update({ items: items, lastId: lastId })
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
            List.map((itm) =>
              <TodoItem key=(string_of_int(TodoItem.getId(itm)))
                        onToggle=(reduce(() => ToggleItem(itm.id)))
                        item=itm />
              , items)
          ));
        }
      };

    <div className="app">
      <div className="title">
        (stringify("What to do"))
        <Input onSubmit=(reduce((text) => AddItem(text))) />
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
