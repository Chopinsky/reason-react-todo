type item = {
  id: int,
  title: string,
  completed: bool
};

let component = ReasonReact.statelessComponent("TodoItem");
let str = ReasonReact.stringToElement;
let newItem = (i, t, c) => { id: i, title: t, completed: c };
let getId = (item) => { item.id };

let make = (~item, ~onToggle, children) => {
  ...component,

  render: (_) => {
    <div className="item" onClick=((_evt) => onToggle())>
      <input
        _type="checkbox"
        checked=(Js.Boolean.to_js_boolean(item.completed))
      />
      (str(item.title))
    </div>
  }
};