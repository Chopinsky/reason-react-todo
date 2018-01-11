type item = {
  title: string,
  completed: bool
};

type state = {
  items: list(item)
};

let component = ReasonReact.reducerComponent("TodoApp");
let stringify = ReasonReact.stringToElement;
let newItem = () => {title: "Click a button", completed: true};

let make = (children) => {
  ...component,

  initialState: () => {
    items: [
      {title: "Write something to do", completed: false}
    ]
  },

  reducer: ((), _) => ReasonReact.NoUpdate,

  render: ({state: {items}}) => {
    let itemCount = List.length(items);
    let itemCountDisplay =
      switch (itemCount) {
      | 1 => "1 item"
      | _ => string_of_int(itemCount) ++ " items"
      };

    <div className="app">
      <div className="title">
        (stringify("What to do"))
        <button onClick=((event) => Js.log("didn't add something, yet..."))>
          (stringify("Add something"))
        </button>
      </div>
      <div className="items"> (stringify("Nothing yet...")) </div>
      <div className="footer">
        (stringify(itemCountDisplay))
      </div>
    </div>
  }
};
