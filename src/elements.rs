use std::rc::Rc;

use gtk::{ glib, traits::{ ButtonExt, ContainerExt, GridExt, RangeExt, ScaleExt, LabelExt, WidgetExt } };

unsafe impl Sync for VolumeSlider {}
unsafe impl Send for VolumeSlider {}
pub struct VolumeSlider {
    volume_label: gtk::Label,
    volume_slider: gtk::Scale,
    mute_button: gtk::Button,
}

impl VolumeSlider {
    pub fn new(
        container: &gtk::Box,
        label: Option<String>,
        start_value: f32,
        muted: bool,
        on_change_vol: Rc<dyn Fn(f32) + 'static>,
        on_change_mute: Rc<dyn Fn() + 'static>
    ) -> VolumeSlider {
        if let Some(label_text) = &label {
            let label = gtk::Label
                ::builder()
                .label(&substring_name(label_text.clone()))
                .halign(gtk::Align::Start)
                .valign(gtk::Align::Start)
                .build();
            container.add(&label);
        }

        let volume_slider = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 1.0);
        volume_slider.set_draw_value(false);
        volume_slider.set_value(start_value as f64);
        volume_slider.connect_change_value(
            move |_, _, d: f64| -> glib::signal::Inhibit {
                on_change_vol(d as f32);
                gtk::Inhibit(false)
            }
        );

        let mute_icon = mute_button_icon(muted);

        let mute_button = gtk::Button::from_icon_name(Some(mute_icon), gtk::IconSize::Button);
        mute_button.connect_clicked(move |_| {
            on_change_mute();
        });

        let volume_label = gtk::Label
            ::builder()
            .label(&start_value.format_volume())
            .build();
        volume_label.set_width_chars(5);

        let grid: gtk::Grid = gtk::Grid::new();
        grid.set_column_spacing(10);
        grid.attach(&volume_label, 0, 0, 2, 3);
        grid.attach_next_to(&volume_slider, Some(&volume_label), gtk::PositionType::Right, 26, 3);
        grid.attach_next_to(&mute_button, Some(&volume_slider), gtk::PositionType::Right, 3, 3);
        container.add(&grid);

        let ret = VolumeSlider {
            volume_label,
            volume_slider,
            mute_button,
        };
        ret.set_grayed_out_slider(muted);
        ret
    }

    pub fn set_volume_slider(&self, value: f32) {
        self.volume_slider.set_value(value as f64);
        self.set_volume_label(value);
    }

    pub fn set_volume_label(&self, value: f32) {
        self.volume_label.set_text_with_mnemonic(&value.format_volume());
    }

    pub fn set_muted(&self, muted: bool) {
        let icon = mute_button_icon(muted);
        self.mute_button.set_image(
            Some(&gtk::Image::from_icon_name(Some(icon), gtk::IconSize::Button))
        );

        self.set_grayed_out_slider(muted);
    }

    fn set_grayed_out_slider(&self, muted: bool) {
        let slider_opacity = if muted { 0.5 } else { 1.0 };
        self.volume_slider.set_opacity(slider_opacity);
        self.volume_label.set_opacity(slider_opacity);
    }
}

pub trait Percentise {
    fn format_volume(&self) -> String;
}

impl Percentise for f32 {
    fn format_volume(&self) -> String {
        format!("{}%", self.round())
    }
}

fn substring_name(name: String) -> String {
    const MAX_NAME_LEN: usize = 30;

    if name.len() < MAX_NAME_LEN {
        return name;
    }
    format!("{}…", &name[..MAX_NAME_LEN])
}

fn mute_button_icon(muted: bool) -> &'static str {
    if muted { "audio-volume-muted" } else { "audio-volume-high" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings() {
        assert_eq!(substring_name("Hello".to_string()), "Hello".to_string());
        assert_eq!(substring_name(
            "Hellooooooooooooooooooooooooooooooo".to_string()),
            "Helloooooooooooooooooooooooooo…".to_string());

        assert_eq!(mute_button_icon(true), "audio-volume-muted");
        assert_eq!(mute_button_icon(false), "audio-volume-high");

        assert_eq!(0.0.format_volume(), "0%");
        assert_eq!(0.1.format_volume(), "0%");
        assert_eq!(500.123123.format_volume(), "500%");
        assert_eq!((-2222.3).format_volume(), "-2222%");
        assert_eq!(0.9.format_volume(), "1%");
    }
}
