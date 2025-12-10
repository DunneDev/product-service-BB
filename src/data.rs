use crate::configuration::Settings;
use crate::model::Product;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Insignia 40\" 1080p HD LCD Fire TV".to_string(),
            price: 199.99,
            description: "Enjoy amazing picture quality and access endless content with the Insignia 40-inch LCD Smart TV. Featuring 1080p HD resolution and direct lit backlight technology, it delivers crisp, lifelike visuals. DTS Virtual-X speaker technology provides immersive sound to complement your viewing experience. The Alexa remote allows for convenient voice control.".to_string(),
            image: "/tv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Apple iPad Air 13\"".to_string(),
            price: 949.97,
            description: "Now available in a 13-inch model, Apple iPad Air is supercharged by the incredibly fast Apple M2 chip. It features a stunning Liquid Retina display, a new landscape camera perfect for FaceTime and video calls, and superfast Wi-Fi 6E2 and 5G. And it works with the new Apple Pencil Pro and Magic Keyboard (each sold separately), so you can multitask, study, work, play, and create from anywhere.".to_string(),
            image: "/ipad.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Corsair Nightsabre".to_string(),
            price: 119.99,
            description: "Rule every battle with speed and precision using the Corsair Nightsabre wireless gaming mouse. Its CORSAIR Marksman 26K dpi optical sensor ensures ultra-precise tracking. Use CORSAIR iCUE software to customize 11 programmable buttons and seven-zone RGB for a personalized setup. QUICKSTRIKE buttons and optical switches deliver lightning-fast clicks in every critical moment.".to_string(),
            image: "/mouse.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Sony WH-1000XM5 Headphones".to_string(),
            price: 378.00,
            description: "Sony’s ultra comfortable WH-1000XM5 headphones rewrite the rules for distraction-free listening and exceptional call clarity. Two processors control multiple microphones for unprecedented noise cancelling while Auto NC Optimizer automatically optimizes noise cancelling based on your wearing conditions and environment. The specially designed 30mm driver unit delivers superior sound quality for all your entertainment.".to_string(),
            image: "/headphones.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Apple Watch SE".to_string(),
            price: 209.99,
            description: "The Apple Watch SE comes equipped with all the essentials to help you be motivated and active, keep connected, track your health and stay safe. The Smart Stack and redesigned apps in watchOS 10 help you see more information at a glance. With features like Crash Detection and enhanced workout metrics, Apple Watch SE is a better value than ever.".to_string(),
            image: "/watch.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Insignia 24\" Freezer Refrigerator".to_string(),
            price: 479.99,
            description: "Keep your fresh and frozen foods organized with a bottom-freezer refrigerator from Insignia. The fridge has a generous 8.3 cu. ft. capacity and features three glass shelves, a crisper drawer, and adjustable door bins. The frost-free 3.18 cu. ft. freezer is located at the bottom of the unit and offers two freezer shelves for smart organization.".to_string(),
            image: "/fridge.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Blendtec Total Blender".to_string(),
            price: 349.99,
            description: "Blend smoothies, crush ice, make ice cream, and a lot more with the Blendtec Total Blender. This commercial-grade blender pulverizes ingredients with its 3.0 peak horsepower motor, and boasts 10 manual speeds and various preprogrammed settings for versatile blending. The extra-large blending jar gives you the freedom to blend large batches.".to_string(),
            image: "/blender.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Hypervolt 2 Percussion Massage Device".to_string(),
            price: 169.99,
            description: "Soothe your tired, aching muscles with the powerful relief of this Hyperice Hypervolt 2 percussion massage device. Ergonomic and lightweight, it’s incredibly easy to use and boasts three speeds for personalised comfort. It’s equipped with QuietGlide technology so you can use the device without disturbing those around you.".to_string(),
            image: "/massage.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Dyson Gen5Detect Vacuum".to_string(),
            price: 899.99,
            description: "Experience a healthy, whole-home deep cleaning with the Dyson Gen5Detect Absolute cordless stick vacuum. It features a washable HEPA filter that picks up even the smallest particles, reducing common allergens and viruses. Its powerful suction technology works on all floor types, and its cordless design makes it ideal for cleaning hard-to-reach areas.".to_string(),
            image: "/vacuum.jpg".to_string()
        },
        Product {
            id: 10,
            name: "PlayStation 5 Slim".to_string(),
            price: 449.99,
            description: "Dominate the game with the PlayStation5 slim digital edition console Fortnite Flowering Chaos bundle. It includes a PS5 digital edition, eight exclusive Fortnite items, and 1,000 V-Bucks to level up your gameplay. Jump into Fortnite with cosmetics like the Florin outfit and Blossom backpack. Explore LEGO Fortnite to build, craft, and adventure in blocky, creative worlds.".to_string(),
            image: "/ps5.jpg".to_string()
        }
    ]
}

