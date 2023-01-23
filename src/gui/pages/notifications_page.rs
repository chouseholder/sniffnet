use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::gui::components::tabs::get_tabs;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::HEIGHT_BODY;
use crate::{RunningPage, Sniffer};
use iced::widget::{Column, Container};
use iced::Length::FillPortion;
use iced::{Alignment, Length};

/// Computes the body of gui notifications page
pub fn notifications_page(sniffer: &Sniffer) -> Container<Message> {
    //let font = get_font(sniffer.style);

    let body = Column::new()
        .width(Length::Fill)
        .padding(5)
        .spacing(5)
        .align_items(Alignment::Center);

    let mut tab_and_body = Column::new().height(FillPortion(HEIGHT_BODY));

    let tabs = get_tabs(
        &["Overview", "Inspect", "Notifications"],
        &["d ", "5 ", "7 "],
        &[
            Message::ChangeRunningPage(RunningPage::Overview),
            Message::ChangeRunningPage(RunningPage::Inspect),
            Message::TickInit,
        ],
        "Notifications",
        sniffer.style,
    );

    tab_and_body = tab_and_body.push(tabs);

    Container::new(Column::new().push(tab_and_body.push(body)))
        .height(FillPortion(HEIGHT_BODY))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}
