// This data was extracted from WireGuard's manuf database
// grep Apple manuf | cut -d' ' -f1 | tr '[:upper:]' '[:lower:]' | tr -d ':' | sed 's/^/"/' | sed 's/$/",/'
pub const APPLE_OUIS: &[&str] = &[
    "000393", "000502", "000a27", "000a95", "000d93", "0010fa", "001124", "001451", "0016cb",
    "0017f2", "0019e3", "001b63", "001cb3", "001d4f", "001e52", "001ec2", "001f5b", "001ff3",
    "0021e9", "002241", "002312", "002332", "00236c", "0023df", "002436", "002500", "00254b",
    "0025bc", "002608", "00264a", "0026b0", "0026bb", "003065", "003ee1", "0050e4", "0056cd",
    "005b94", "006171", "006d52", "007d60", "00812a", "008865", "008a76", "00a040", "00b362",
    "00c585", "00c610", "00cdfe", "00db70", "00f39f", "00f4b9", "00f76f", "040cce", "04137a",
    "041552", "041e64", "042665", "0441a5", "04489a", "044bed", "0452f3", "045453", "046865",
    "0469f8", "047295", "0499b9", "0499bb", "049d05", "04bc6d", "04bfd5", "04d3cf", "04db56",
    "04e536", "04f13e", "04f7e4", "080007", "082573", "082cb6", "085d53", "086202", "086518",
    "086698", "086d41", "087045", "087402", "0887c7", "088edc", "089542", "08c729", "08c7b5",
    "08e689", "08f4ab", "08f69c", "08f8bc", "08ff44", "0c1539", "0c1563", "0c19f8", "0c3021",
    "0c3b50", "0c3e9f", "0c4de9", "0c5101", "0c517e", "0c53b7", "0c6ac4", "0c74c2", "0c771a",
    "0c85e1", "0cbc9f", "0cc56c", "0cd746", "0cdbea", "0ce441", "100020", "101c0c", "102959",
    "102fca", "103025", "1040f3", "10417f", "1093e9", "1094bb", "109add", "109f41", "10a2d3",
    "10b588", "10b9c4", "10bd3a", "10cee9", "10cf0f", "10da63", "10ddb1", "10e2c9", "14109f",
    "14147d", "141a97", "141ba0", "14205e", "142876", "142d4d", "1435b7", "145a05", "1460cb",
    "147dda", "147fce", "148509", "14876a", "1488e6", "148fc6", "14946c", "1495ce", "149877",
    "1499e2", "149d99", "14bd61", "14c213", "14c88b", "14d00d", "14d19e", "14f287", "182032",
    "183451", "183eef", "183f70", "184a53", "1855e3", "1856c3", "186590", "187eb9", "18810e",
    "189efc", "18af61", "18af8f", "18e7b0", "18e7f4", "18ee69", "18f1d8", "18f643", "18fab7",
    "1c0d7d", "1c0ec2", "1c1ac0", "1c1dd3", "1c36bb", "1c3c78", "1c57dc", "1c5cf2", "1c61bf",
    "1c6a76", "1c7125", "1c8682", "1c9148", "1c9180", "1c9e46", "1caba7", "1cb3c9", "1ce209",
    "1ce62b", "1cf64c", "1cf9d5", "200484", "200e2b", "201582", "201a94", "202df6", "2032c6",
    "2037a5", "203cae", "206980", "20768f", "2078cd", "2078f0", "207d74", "2091df", "209bcd",
    "20a2e4", "20a5cb", "20ab37", "20c9d0", "20e2a8", "20e874", "20ee28", "20fa85", "241b7a",
    "241eeb", "24240e", "245ba7", "245e48", "24a074", "24a2e1", "24ab81", "24b339", "24d0df",
    "24e314", "24f094", "24f677", "28022e", "280244", "280b5c", "282d7f", "2834ff", "283737",
    "285aeb", "286ab8", "286aba", "2877f1", "2883c9", "288eec", "288ff6", "28a02b", "28c1a0",
    "28c538", "28c709", "28cfda", "28cfe9", "28e02c", "28e14c", "28e7cf", "28ea2d", "28ec95",
    "28ed6a", "28f033", "28f076", "28ff3c", "2c1809", "2c1f23", "2c200b", "2c326a", "2c3361",
    "2c57ce", "2c61f6", "2c7600", "2c7cf2", "2c81bf", "2c8217", "2cb43a", "2cbc87", "2cbe08",
    "2cc253", "2cca16", "2cf0a2", "2cf0ee", "3010e4", "3035ad", "303b7c", "305714", "30636b",
    "308216", "309048", "3090ab", "30d53e", "30d7a1", "30d875", "30d9d9", "30e04f", "30f7c5",
    "3408bc", "341298", "34159e", "342840", "342b6e", "34318f", "34363b", "344262", "3451c9",
    "346691", "347c25", "348c5e", "34a395", "34a8eb", "34ab37", "34b1eb", "34c059", "34e2fd",
    "34ee16", "34f68d", "34fd6a", "34fe77", "3809fb", "380f4a", "38484c", "38539c", "386233",
    "3865b2", "3866f0", "3871de", "387f8b", "3888a4", "38892c", "389cb2", "38b54d", "38c43a",
    "38c986", "38cada", "38e13d", "38ec0d", "38f9d3", "3c0630", "3c0754", "3c07d7", "3c15c2",
    "3c1eb5", "3c22fb", "3c2ef9", "3c2eff", "3c39c8", "3c3b77", "3c4dbe", "3c5002", "3c6d89",
    "3c7d0a", "3ca6f6", "3cab8e", "3cbf60", "3ccd36", "3cd0f8", "3cdd57", "3ce072", "402619",
    "403004", "40331a", "403cfc", "404d7f", "406c8f", "4070f5", "407911", "40831d", "40921a",
    "4098ad", "409c28", "40a6d9", "40b395", "40b3fa", "40bc60", "40c711", "40cbc0", "40d160",
    "40d32d", "40da5c", "40e64b", "40edcf", "40f946", "440010", "4409da", "4418fd", "441b88",
    "442a60", "443583", "444adb", "444c0c", "4490bb", "449e8b", "44a10e", "44a7f4", "44a8fc",
    "44c65d", "44d884", "44da30", "44e66e", "44f09e", "44f21b", "44fb42", "48262c", "48352b",
    "483b38", "48437c", "484baa", "4860bc", "48746e", "48a195", "48a91c", "48b8a3", "48bf6b",
    "48d705", "48e15c", "48e9f1", "4c20b8", "4c2eb4", "4c3275", "4c569d", "4c57ca", "4c5d6a",
    "4c6be8", "4c74bf", "4c7975", "4c7c5f", "4c7cd9", "4c8d79", "4c97cc", "4c9ff1", "4cab4f",
    "4cb199", "4cb910", "4ccdb6", "4ce6c0", "501fc6", "5023a2", "503237", "50578a", "507a55",
    "507ac5", "5082d5", "50a67f", "50a6d8", "50b127", "50bc96", "50de06", "50ead6", "50ed3c",
    "50f265", "50f351", "50f4eb", "540910", "542696", "542906", "542b8d", "5432c7", "5433cb",
    "544e90", "5462e2", "54724f", "549963", "549f13", "54ae27", "54e43a", "54e61b", "54eaa8",
    "54ebe9", "580ad4", "581faa", "583653", "58404e", "585595", "5855ca", "5864c4", "58666d",
    "586b14", "5873d8", "587f57", "5893e8", "58ad12", "58b035", "58b965", "58d349", "58e28f",
    "58e6ba", "5c0947", "5c13cc", "5c1bf4", "5c1dd9", "5c3e1b", "5c50d9", "5c5230", "5c5284",
    "5c5948", "5c7017", "5c8730", "5c8d4e", "5c9175", "5c95ae", "5c969d", "5c97f3", "5c9ba6",
    "5cadba", "5cadcf", "5ce91e", "5cf5da", "5cf7e6", "5cf938", "600308", "6006e3", "600f6b",
    "6030d4", "60334b", "603e5f", "6057c8", "606525", "606944", "6070c0", "607ec9", "608110",
    "608246", "608373", "608b0e", "608c4a", "609217", "609316", "6095bd", "609ac1", "60a37d",
    "60bec4", "60c547", "60d039", "60d9c7", "60dd70", "60f445", "60f549", "60f81d", "60facd",
    "60fb42", "60fda6", "60fec5", "640bd7", "640c91", "64200c", "643135", "6441e6", "644842",
    "645a36", "645aed", "646d2f", "647033", "6476ba", "649abe", "64a3cb", "64a5c3", "64b0a6",
    "64b9e8", "64c753", "64d2c4", "64e682", "680927", "682f67", "683ec0", "684465", "6845cc",
    "685b35", "68644b", "6883cb", "68967b", "689c70", "68a86d", "68ab1e", "68ae20", "68cac4",
    "68d93c", "68dbca", "68e580", "68ef43", "68efdc", "68fb7e", "68fef7", "6c1270", "6c19c0",
    "6c1f8a", "6c3aff", "6c3e6d", "6c4008", "6c4a85", "6c4d73", "6c709f", "6c72e7", "6c7e67",
    "6c8dc1", "6c94f8", "6c96cf", "6cab31", "6cb133", "6cc26b", "6ce5c9", "6ce85c", "701124",
    "701384", "7014a6", "7022fe", "70317f", "703c69", "703eac", "70480f", "705681", "70700d",
    "7072fe", "7073cb", "7081eb", "708cf2", "70a2b3", "70aed5", "70b306", "70bb5b", "70cd60",
    "70dee2", "70e72c", "70ea5a", "70ece4", "70ef00", "70f087", "70f94a", "740ea4", "7415f5",
    "741bb2", "743174", "744218", "74428b", "74650c", "74718b", "7473b4", "748114", "748d08",
    "748f3c", "749eaf", "74a6cd", "74b587", "74cc40", "74e1b6", "74e2f5", "78028b", "7831c1",
    "783a84", "783f4d", "784f43", "785ecc", "7864c0", "7867d7", "786c1c", "787b8a", "787e61",
    "78886d", "789f70", "78a3e4", "78a7c7", "78ca39", "78d162", "78d75f", "78e3de", "78fbd8",
    "78fd94", "7c0191", "7c04d0", "7c11be", "7c2499", "7c296f", "7c2aca", "7c3b2d", "7c4b26",
    "7c5049", "7c6130", "7c6d62", "7c6df8", "7c9a1d", "7ca1ae", "7cab60", "7cc06f", "7cc180",
    "7cc3a1", "7cc537", "7cc8df", "7cd1c3", "7cd2da", "7cecb1", "7cf05f", "7cf34d", "7cfadf",
    "7cfc16", "80006e", "80045f", "800c67", "804971", "804a14", "8054e3", "805fc5", "80657c",
    "808223", "8083f6", "80929f", "80953a", "809698", "80a997", "80af19", "80b03d", "80b989",
    "80be05", "80d605", "80e650", "80ea96", "80ed2c", "840f4c", "842999", "842f57", "843835",
    "844167", "846878", "84788b", "848506", "8488e1", "8489ad", "848c8d", "848e0c", "849437",
    "84a134", "84ab1a", "84ac16", "84ad8d", "84b153", "84b1e4", "84d328", "84fcac", "84fcfe",
    "881908", "881e5a", "881fa1", "88200d", "884d7c", "8851f2", "885395", "8863df", "886440",
    "88665a", "8866a5", "886b6e", "886bdb", "88a479", "88a9b7", "88ae07", "88b291", "88b7eb",
    "88b945", "88c08b", "88c663", "88cb87", "88e87f", "88e9fe", "8c006d", "8c08aa", "8c26aa",
    "8c2937", "8c2daa", "8c3396", "8c5877", "8c7aaa", "8c7b9d", "8c7c92", "8c8590", "8c861e",
    "8c8ef2", "8c8fe9", "8c986b", "8cec7b", "8cfaba", "8cfe57", "9027e4", "902c09", "903c92",
    "904cc5", "905f7a", "9060f1", "90623f", "907240", "90812a", "908158", "90840d", "908c43",
    "908d6c", "909b6f", "909c4a", "90a25b", "90b0ed", "90b21f", "90b790", "90b931", "90c1c6",
    "90cde8", "90dd5d", "90e17b", "90ecea", "90fd61", "940bcd", "940c98", "941625", "942157",
    "942b68", "943fd6", "945c9a", "949426", "94ad23", "94b01f", "94bf2d", "94e96a", "94ea32",
    "94f6a3", "94f6d6", "9800c6", "9801a7", "9803d8", "980daf", "9810e8", "981ca2", "983c8c",
    "98460a", "98502e", "985aeb", "9860ca", "98698a", "989e63", "98a5f9", "98b379", "98b8e3",
    "98ca33", "98d6bb", "98dd60", "98e0d9", "98f0ab", "98fe94", "98fee1", "9c04eb", "9c1a25",
    "9c207b", "9c28b3", "9c293f", "9c35eb", "9c3e53", "9c4fda", "9c583c", "9c5884", "9c6076",
    "9c648b", "9c760e", "9c79e3", "9c84bf", "9c8ba0", "9c924f", "9ca9c5", "9cdaa8", "9ce33f",
    "9ce65e", "9cf387", "9cf3ac", "9cf48e", "9cfa76", "9cfc01", "9cfc28", "a01828", "a03be3",
    "a04ea7", "a04ecf", "a05272", "a056f3", "a07817", "a0782d", "a0999b", "a09a8e", "a09d22",
    "a0a309", "a0b40f", "a0d1b3", "a0d795", "a0edcd", "a0ee1a", "a0fbc5", "a40987", "a416c0",
    "a43135", "a45e60", "a46706", "a477f3", "a483e7", "a4b197", "a4b805", "a4c337", "a4c361",
    "a4c6f0", "a4cf99", "a4d18c", "a4d1d2", "a4d23e", "a4d931", "a4e975", "a4f1e8", "a4f6e8",
    "a4f841", "a4f921", "a4fc14", "a81af1", "a82066", "a84a28", "a851ab", "a85b78", "a85bb7",
    "a85c2c", "a860b6", "a8667f", "a87cf8", "a8817e", "a886dd", "a88808", "a88e24", "a88fd9",
    "a8913d", "a8968a", "a89c78", "a8abb5", "a8bb56", "a8bbcf", "a8be27", "a8fad8", "a8fe9d",
    "ac007a", "ac0775", "ac15f4", "ac1615", "ac1d06", "ac1f74", "ac293a", "ac3c0b", "ac4500",
    "ac49db", "ac5c2c", "ac61ea", "ac7f3e", "ac86a3", "ac87a3", "ac88fd", "ac9085", "ac9738",
    "acbc32", "acbcb5", "acc906", "accf5c", "acdfa1", "ace4b5", "acfdec", "b01831", "b019c6",
    "b03495", "b035b5", "b03f64", "b0481a", "b065bd", "b067b5", "b0702d", "b08c75", "b09200",
    "b09fba", "b0be83", "b0ca68", "b0d576", "b0de28", "b0e5ef", "b0e5f9", "b0f1d8", "b418d1",
    "b41974", "b41bb0", "b440a4", "b44bd2", "b456e3", "b45976", "b485e1", "b48b19", "b496a5",
    "b49cdf", "b4aec1", "b4f0ab", "b4f61c", "b4fa48", "b8098a", "b8144d", "b817c2", "b8211c",
    "b8220c", "b82aa9", "b8374a", "b83c28", "b841a4", "b844d9", "b845eb", "b8496d", "b853ac",
    "b85d0a", "b8634d", "b8782e", "b87bc5", "b881fa", "b88d12", "b89047", "b8b2f8", "b8c111",
    "b8c75d", "b8e60c", "b8e856", "b8f12a", "b8f6b1", "b8ff61", "bc0963", "bc37d3", "bc3baf",
    "bc4cc4", "bc52b7", "bc5436", "bc6778", "bc6c21", "bc804e", "bc89a7", "bc926b", "bc9f58",
    "bc9fef", "bca5a9", "bca920", "bcb863", "bcbb58", "bcd074", "bce143", "bcec5d", "bcfed9",
    "c01754", "c01ada", "c02c5c", "c04442", "c06394", "c06c0c", "c0847a", "c0956d", "c09ad0",
    "c09f42", "c0a53e", "c0a600", "c0b22f", "c0b658", "c0c7db", "c0ccf8", "c0cecd", "c0d012",
    "c0e862", "c0f2fb", "c40b31", "c41234", "c41411", "c42ad0", "c42c03", "c435d9", "c4524f",
    "c4618b", "c48466", "c484fc", "c4910c", "c49880", "c4acaa", "c4b301", "c4b349", "c4c17d",
    "c4c36b", "c4f7c1", "c81ee7", "c81fe8", "c82a14", "c8334b", "c83c85", "c869cd", "c86f1d",
    "c88550", "c889f3", "c8b1cd", "c8b5b7", "c8bcc8", "c8d083", "c8e0eb", "c8f650", "cc088d",
    "cc08e0", "cc08fa", "cc115a", "cc20e8", "cc25ef", "cc2746", "cc29f5", "cc2db7", "cc3f36",
    "cc4463", "cc4b04", "cc6023", "cc660a", "cc68e0", "cc69fa", "cc785f", "cc817d", "ccc760",
    "ccc95d", "ccd281", "d0034b", "d011e5", "d023db", "d02598", "d02b20", "d03311", "d03e07",
    "d03faa", "d04d86", "d04f7e", "d058a5", "d06544", "d06b78", "d0817a", "d0880c", "d0a637",
    "d0c050", "d0c5f3", "d0d23c", "d0d2b0", "d0d49f", "d0dad7", "d0e140", "d0e581", "d40f9e",
    "d42fca", "d446e1", "d45763", "d4619d", "d461da", "d468aa", "d4909c", "d49a20", "d4a33d",
    "d4dccd", "d4f46f", "d4fb8e", "d8004d", "d81c79", "d81d72", "d83062", "d84c90", "d87475",
    "d88f76", "d89695", "d89e3f", "d8a25e", "d8bb2c", "d8be1f", "d8cf9c", "d8d1cb", "d8dc40",
    "d8de3a", "d8e593", "dc080f", "dc0c5c", "dc1057", "dc2b2a", "dc2b61", "dc3714", "dc415f",
    "dc45b8", "dc5285", "dc5392", "dc56e7", "dc6dbc", "dc71d0", "dc8084", "dc86d8", "dc9566",
    "dc9b9c", "dc9e8f", "dca4ca", "dca904", "dcb54f", "dcd3a2", "dcf4ca", "e02b96", "e0338e",
    "e05f45", "e06678", "e06d17", "e0897e", "e0925c", "e0accb", "e0b52d", "e0b55f", "e0b9ba",
    "e0bda0", "e0bfb2", "e0c3ea", "e0c767", "e0c97a", "e0eb40", "e0f5c6", "e0f847", "e425e7",
    "e42b34", "e450eb", "e47684", "e48b7f", "e490fd", "e498d6", "e49a79", "e49adc", "e49c67",
    "e4b2fb", "e4c63d", "e4ce8f", "e4e0a6", "e4e4ab", "e8040b", "e80688", "e81cd8", "e83617",
    "e84a78", "e85f02", "e87865", "e87f95", "e8802e", "e88152", "e8854b", "e88d28", "e8a730",
    "e8b2ac", "e8fbe9", "e8fff4", "ec0d51", "ec2651", "ec28d3", "ec2c73", "ec2ce2", "ec3586",
    "ec42cc", "ec4654", "ec7379", "ec8150", "ec852f", "ec97a2", "eca907", "ecadb8", "ecced7",
    "ecff3a", "f004e1", "f01898", "f01fc7", "f02475", "f027a0", "f02f4b", "f05cd5", "f0766f",
    "f07807", "f07960", "f0989d", "f099b6", "f099bf", "f0a35a", "f0b0e7", "f0b3ec", "f0b479",
    "f0c1f1", "f0c371", "f0c725", "f0cba1", "f0d1a9", "f0d31f", "f0d635", "f0d793", "f0dbe2",
    "f0dbf8", "f0dce2", "f0ee7a", "f0f61c", "f40616", "f40e01", "f40f24", "f41ba1", "f421ca",
    "f431c3", "f434f0", "f437b7", "f439a6", "f45293", "f45c89", "f465a6", "f4a310", "f4afe7",
    "f4beec", "f4cbe7", "f4d488", "f4dbe3", "f4e8c7", "f4f15a", "f4f951", "f4fe3e", "f80377",
    "f81093", "f81edf", "f82793", "f82d7c", "f83880", "f84288", "f84d89", "f84e73", "f86214",
    "f8665a", "f86fc1", "f871a6", "f873df", "f87d76", "f887f1", "f895ea", "f8b1dd", "f8c3cc",
    "f8e5ce", "f8e94e", "f8f58c", "f8ffc2", "fc183c", "fc1d43", "fc253f", "fc2a9c", "fc315d",
    "fc47d8", "fc4ea4", "fc5557", "fc66cf", "fc9ca7", "fca5c8", "fcaa81", "fcb6d8", "fcd848",
    "fce26c", "fce998", "fcfc48",
];
