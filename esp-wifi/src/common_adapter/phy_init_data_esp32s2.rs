use crate::binary::include::esp_phy_init_data_t;

const CONFIG_ESP_PHY_MAX_TX_POWER: u8 = 20;

const fn limit(val: u8, low: u8, high: u8) -> u8 {
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}

pub(crate) static PHY_INIT_DATA_DEFAULT: esp_phy_init_data_t = esp_phy_init_data_t {
    params: [
        0x80,
        0x00,
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x4E),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x4E),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x48),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x48),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x48),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x48),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x48),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x48),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x44),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x44),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x48),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x48),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x44),
        limit(CONFIG_ESP_PHY_MAX_TX_POWER * 4, 0, 0x42),
        0x00,
        0x00,
        0x00,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0xff,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0xf1,
    ],
};
