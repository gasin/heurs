use yew::prelude::*;

/// A trait for items that can be displayed in the list.
/// It requires items to have an ID.
pub trait ListItem {
    fn id(&self) -> i32;
}

#[derive(Properties, PartialEq)]
pub struct Props<T: ListItem + PartialEq> {
    pub title: String,
    pub items: Vec<T>,
    pub selected_id: Option<i32>,
    pub on_select: Callback<i32>,
    pub headers: Vec<String>,
    pub render_item_row: Callback<T, Html>,
}

/// A generic component to display a selectable list of items in a panel.
/// It mirrors the styling and behavior of the original `test_cases` page list.
#[function_component(ItemListPanel)]
pub fn item_list_panel<T>(props: &Props<T>) -> Html
where
    T: ListItem + Clone + PartialEq + 'static,
{
    html! {
        <div style="width:45%;">
            <h2>{ &props.title }</h2>
            <div style="max-height:80vh; overflow-y:auto; border:1px solid #ccc;">
                <table style="width:100%;border-collapse:collapse;">
                    <thead>
                        <tr>
                            { for props.headers.iter().map(|h| html! {
                                <th style="border-bottom:1px solid #ccc;padding:4px;text-align:left;">{ h }</th>
                            })}
                        </tr>
                    </thead>
                    <tbody>
                        { for props.items.iter().cloned().map(|item| {
                            let id = item.id();
                            let on_select = props.on_select.clone();
                            let onclick = Callback::from(move |_| on_select.emit(id));

                            let is_selected = props.selected_id == Some(id);
                            // This styling logic is taken directly from the test_cases page
                            let base_style = "cursor:pointer; transition:background-color 0.2s ease;";
                            let style = if is_selected {
                                format!("{} {}", base_style, "background-color:#d0e0ff;")
                            } else {
                                base_style.to_string()
                            };

                            html! {
                                <tr {onclick} {style}>
                                    { props.render_item_row.emit(item.clone()) }
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>
        </div>
    }
}
