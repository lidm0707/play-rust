use yew::prelude::*;
#[function_component(InvoiceForm)]
pub fn invoice_form() -> Html {
    

    html! {
        <form >
            <div>
                <label for="name">{"Name:"}</label>
                <input type="text" id="name" name="name"   required=true />
            </div>
            <div>
                <label for="address">{"Address:"}</label>
                <input type="text" id="address" name="address"  required=true />
            </div>
            <div>
                <label for="taxId">{"Tax ID:"}</label>
                <input type="text" id="taxId" name="taxId"  required=true />
            </div>
            <div>
                <label for="vatType">{"VAT Type:"}</label>
                <select id="vatType" name="vatType"  required=true>
                    <option value="standard">{"Standard"}</option>
                    <option value="reduced">{"Reduced"}</option>
                    <option value="zero">{"Zero"}</option>
                </select>
            </div>
            <div>
                <label for="product">{"Product:"}</label>
                <input type="text" id="product" name="product"  required=true />
            </div>
            <div>
                <label for="price">{"Price:"}</label>
                <input type="number" id="price" name="price" step="0.01" required=true />
            </div>
            <div>
                <label for="quantity">{"Quantity:"}</label>
                <input type="range" id="quantity" name="quantity" min="1" max="10"  />
                <span></span>
            </div>
            <div>
                <h3>{"Summary"}</h3>
                <p>{"Total Price: "}</p>
                <p>{"Total Tax: "}</p>
            </div>
            <button type="submit">{"Save to PDF"}</button>
        </form>
    }
}


