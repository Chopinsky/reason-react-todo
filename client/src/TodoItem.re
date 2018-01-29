type item = {
  title: string,
  completed: bool
};

let component = ReasonReact.statelessComponent("TodoItem");
let str = ReasonReact.stringToElement;

let make = (~item, children) => {
  ...component,

  render: (self) => {
    <div className="item">
      <input
        _type="checkbox"
        checked=(Js.Boolean.to_js_boolean(item.completed))
        /* TODO make interactive */
      />
      (str(item.title))
    </div>
  }
};