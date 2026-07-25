use aes::Aes128;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, KeyInit};
use jni::objects::{JClass, JString};
use jni::sys::{jbyteArray, jstring};
use jni::{JNIEnv, JavaVM};
use md5::compute;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
use uuid::Uuid;

// ===== Constants: Trans character maps =====
// Simplified -> Traditional mapping (selected common chars)
static S2T_DATA: &[(&str, &str)] = &[
    ("万", "萬"), ("与", "與"), ("丑", "醜"), ("东", "東"), ("丝", "絲"),
    ("严", "嚴"), ("丧", "喪"), ("个", "個"), ("丰", "豐"), ("临", "臨"),
    ("丽", "麗"), ("举", "舉"), ("么", "麼"), ("乌", "烏"), ("乐", "樂"),
    ("乔", "喬"), ("习", "習"), ("乡", "鄉"), ("书", "書"), ("买", "買"),
    ("乱", "亂"), ("争", "爭"), ("于", "於"), ("亏", "虧"), ("云", "雲"),
    ("亚", "亞"), ("产", "產"), ("亩", "畝"), ("亲", "親"), ("亿", "億"),
    ("仅", "僅"), ("从", "從"), ("仓", "倉"), ("仪", "儀"), ("们", "們"),
    ("价", "價"), ("众", "眾"), ("优", "優"), ("伙", "夥"), ("会", "會"),
    ("伞", "傘"), ("伟", "偉"), ("传", "傳"), ("伤", "傷"), ("伦", "倫"),
    ("伪", "偽"), ("体", "體"), ("余", "餘"), ("佣", "傭"), ("侠", "俠"),
    ("侣", "侶"), ("侦", "偵"), ("侧", "側"), ("侨", "僑"), ("债", "債"),
    ("倾", "傾"), ("偿", "償"), ("储", "儲"), ("儿", "兒"), ("兑", "兌"),
    ("党", "黨"), ("兰", "蘭"), ("关", "關"), ("兴", "興"), ("兹", "茲"),
    ("养", "養"), ("兽", "獸"), ("内", "內"), ("冈", "岡"), ("册", "冊"),
    ("写", "寫"), ("军", "軍"), ("农", "農"), ("冯", "馮"), ("冲", "衝"),
    ("决", "決"), ("况", "況"), ("冻", "凍"), ("净", "淨"), ("减", "減"),
    ("凤", "鳳"), ("凯", "凱"), ("击", "擊"), ("凿", "鑿"), ("划", "劃"),
    ("刘", "劉"), ("则", "則"), ("刚", "剛"), ("创", "創"), ("删", "刪"),
    ("别", "別"), ("刹", "剎"), ("剂", "劑"), ("剑", "劍"), ("务", "務"),
    ("动", "動"), ("励", "勵"), ("劲", "勁"), ("劳", "勞"), ("势", "勢"),
    ("勋", "勳"), ("匀", "勻"), ("医", "醫"), ("华", "華"), ("协", "協"),
    ("单", "單"), ("卖", "賣"), ("卢", "盧"), ("卤", "鹵"), ("卧", "臥"),
    ("卫", "衛"), ("却", "卻"), ("厂", "廠"), ("厅", "廳"), ("历", "曆"),
    ("压", "壓"), ("厕", "廁"), ("厦", "廈"), ("厨", "廚"), ("县", "縣"),
    ("参", "參"), ("双", "雙"), ("发", "發"), ("变", "變"), ("叶", "葉"),
    ("号", "號"), ("叹", "嘆"), ("吁", "籲"), ("吓", "嚇"), ("吗", "嗎"),
    ("吨", "噸"), ("听", "聽"), ("吴", "吳"), ("员", "員"), ("呜", "嗚"),
    ("咏", "詠"), ("咙", "嚨"), ("响", "響"), ("哑", "啞"), ("哗", "嘩"),
    ("哟", "喲"), ("唤", "喚"), ("喷", "噴"), ("喽", "嘍"), ("啸", "嘯"),
    ("啰", "囉"), ("嘘", "噓"), ("嘱", "囑"), ("团", "團"), ("园", "園"),
    ("围", "圍"), ("国", "國"), ("图", "圖"), ("圆", "圓"), ("圣", "聖"),
    ("场", "場"), ("坏", "壞"), ("坚", "堅"), ("坛", "壇"), ("坝", "壩"),
    ("坞", "塢"), ("坟", "墳"), ("坠", "墜"), ("垄", "壟"), ("垦", "墾"),
    ("垫", "墊"), ("墙", "牆"), ("壮", "壯"), ("声", "聲"), ("壳", "殼"),
    ("壶", "壺"), ("处", "處"), ("备", "備"), ("复", "復"), ("够", "夠"),
    ("头", "頭"), ("夸", "誇"), ("夺", "奪"), ("奋", "奮"), ("奖", "獎"),
    ("奥", "奧"), ("妆", "妝"), ("妇", "婦"), ("妈", "媽"), ("姜", "薑"),
    ("娄", "婁"), ("娅", "婭"), ("娆", "嬈"), ("娇", "嬌"), ("娱", "娛"),
    ("娲", "媧"), ("婴", "嬰"), ("婵", "嬋"), ("婶", "嬸"), ("孙", "孫"),
    ("学", "學"), ("宁", "寧"), ("宝", "寶"), ("实", "實"), ("宠", "寵"),
    ("审", "審"), ("宪", "憲"), ("宫", "宮"), ("宽", "寬"), ("宾", "賓"),
    ("寻", "尋"), ("导", "導"), ("寿", "壽"), ("尔", "爾"), ("尘", "塵"),
    ("尧", "堯"), ("尸", "屍"), ("尽", "盡"), ("属", "屬"), ("岁", "歲"),
    ("岛", "島"), ("岭", "嶺"), ("岳", "嶽"), ("岸", "岸"), ("峡", "峽"),
    ("峦", "巒"), ("峰", "峰"), ("峡", "峽"), ("崔", "崔"), ("崖", "崖"),
    ("崩", "崩"), ("崭", "嶄"), ("嵌", "嵌"), ("嵘", "嶸"), ("嵩", "嵩"),
    ("崭", "嶄"), ("岳", "嶽"), ("岷", "岷"), ("岐", "岐"), ("岑", "岑"),
    ("岔", "岔"), ("岗", "崗"), ("岛", "島"), ("崇", "崇"), ("崎", "崎"),
    ("崔", "崔"), ("崖", "崖"), ("崩", "崩"), ("嵌", "嵌"), ("嵘", "嶸"),
    ("崭", "嶄"), ("岳", "嶽"), ("巅", "巔"), ("岩", "巖"), ("巩", "鞏"),
    ("币", "幣"), ("帅", "帥"), ("师", "師"), ("帐", "帳"), ("帘", "簾"),
    ("帜", "幟"), ("带", "帶"), ("帧", "幀"), ("帮", "幫"), ("并", "並"),
    ("广", "廣"), ("庄", "莊"), ("庆", "慶"), ("庐", "廬"), ("库", "庫"),
    ("应", "應"), ("庙", "廟"), ("庞", "龐"), ("废", "廢"), ("开", "開"),
    ("异", "異"), ("弃", "棄"), ("张", "彌"), ("弥", "彌"), ("弯", "彎"),
    ("弹", "彈"), ("强", "強"), ("归", "歸"), ("当", "當"), ("彦", "彥"),
    ("彻", "徹"), ("径", "徑"), ("御", "禦"), ("忆", "憶"), ("忏", "懺"),
    ("忧", "憂"), ("怀", "懷"), ("态", "態"), ("怂", "慫"), ("怅", "悵"),
    ("怜", "憐"), ("总", "總"), ("恋", "戀"), ("恳", "懇"), ("恶", "惡"),
    ("恼", "惱"), ("悦", "悅"), ("悬", "懸"), ("悯", "憫"), ("惊", "驚"),
    ("惧", "懼"), ("惨", "慘"), ("惩", "懲"), ("惫", "憊"), ("惭", "慚"),
    ("惮", "憚"), ("惯", "慣"), ("愤", "憤"), ("慑", "懾"), ("懒", "懶"),
    ("戏", "戲"), ("战", "戰"), ("扎", "紮"), ("扑", "撲"), ("执", "執"),
    ("扩", "擴"), ("扪", "捫"), ("扫", "掃"), ("扬", "揚"), ("扰", "擾"),
    ("抚", "撫"), ("抛", "拋"), ("抠", "摳"), ("抡", "掄"), ("抢", "搶"),
    ("护", "護"), ("报", "報"), ("担", "擔"), ("拟", "擬"), ("拢", "攏"),
    ("拣", "揀"), ("拥", "擁"), ("拦", "攔"), ("拧", "擰"), ("拨", "撥"),
    ("择", "擇"), ("挂", "掛"), ("挚", "摯"), ("挛", "攣"), ("挞", "撻"),
    ("挟", "挾"), ("挠", "撓"), ("挡", "擋"), ("挣", "掙"), ("挤", "擠"),
    ("挥", "揮"), ("捞", "撈"), ("损", "損"), ("捡", "撿"), ("换", "換"),
    ("捣", "搗"), ("据", "據"), ("拣", "揀"), ("掳", "擄"), ("掷", "擲"),
    ("掺", "摻"), ("揽", "攬"), ("搀", "攙"), ("搁", "擱"), ("搂", "摟"),
    ("搅", "攪"), ("携", "攜"), ("摄", "攝"), ("摆", "擺"), ("摇", "搖"),
    ("摊", "攤"), ("撑", "撐"), ("撵", "攆"), ("撷", "擷"), ("撸", "擼"),
    ("攒", "攢"), ("敌", "敵"), ("敛", "斂"), ("数", "數"), ("斋", "齋"),
    ("断", "斷"), ("旧", "舊"), ("时", "時"), ("旷", "曠"), ("昙", "曇"),
    ("昼", "晝"), ("显", "顯"), ("晋", "晉"), ("晒", "曬"), ("晓", "曉"),
    ("晕", "暈"), ("暂", "暫"), ("术", "術"), ("朴", "樸"), ("机", "機"),
    ("杀", "殺"), ("杂", "雜"), ("权", "權"), ("条", "條"), ("杨", "楊"),
    ("杰", "傑"), ("极", "極"), ("构", "構"), ("枢", "樞"), ("枣", "棗"),
    ("枫", "楓"), ("柜", "櫃"), ("柠", "檸"), ("栀", "梔"), ("栅", "柵"),
    ("标", "標"), ("栈", "棧"), ("栋", "棟"), ("栏", "欄"), ("树", "樹"),
    ("栖", "棲"), ("样", "樣"), ("栓", "栓"), ("档", "檔"), ("桥", "橋"),
    ("桦", "樺"), ("桧", "檜"), ("桩", "樁"), ("梦", "夢"), ("检", "檢"),
    ("楼", "樓"), ("桨", "槳"), ("棚", "棚"), ("椭", "橢"), ("楼", "樓"),
    ("榈", "櫚"), ("榴", "榴"), ("槊", "槊"), ("槛", "檻"), ("槟", "檳"),
    ("柠", "檸"), ("树", "樹"), ("桦", "樺"), ("桧", "檜"), ("桩", "樁"),
    ("梦", "夢"), ("检", "檢"), ("榄", "欖"), ("楼", "樓"), ("样", "樣"),
    ("栾", "欒"), ("榄", "欖"), ("椭", "橢"), ("楼", "樓"), ("榈", "櫚"),
    ("榴", "榴"), ("槊", "槊"), ("槛", "檻"), ("槟", "檳"), ("柠", "檸"),
    ("橱", "櫥"), ("橹", "櫓"), ("檐", "簷"), ("欢", "歡"), ("欧", "歐"),
    ("歼", "殲"), ("残", "殘"), ("殒", "殞"), ("殡", "殯"), ("殴", "毆"),
    ("毁", "毀"), ("毂", "轂"), ("毙", "斃"), ("毡", "氈"), ("气", "氣"),
    ("氢", "氫"), ("氩", "氬"), ("氲", "氳"), ("汉", "漢"), ("汤", "湯"),
    ("汹", "洶"), ("沟", "溝"), ("没", "沒"), ("沣", "灃"), ("沥", "瀝"),
    ("沦", "淪"), ("沧", "滄"), ("沪", "滬"), ("泞", "濘"), ("泪", "淚"),
    ("泻", "瀉"), ("泼", "潑"), ("泽", "澤"), ("泾", "涇"), ("洁", "潔"),
    ("洒", "灑"), ("浇", "澆"), ("浊", "濁"), ("测", "測"), ("济", "濟"),
    ("浏", "瀏"), ("浑", "渾"), ("浓", "濃"), ("涌", "湧"), ("涛", "濤"),
    ("涝", "澇"), ("涨", "漲"), ("涩", "澀"), ("淀", "澱"), ("渊", "淵"),
    ("渍", "漬"), ("渐", "漸"), ("渔", "漁"), ("渗", "滲"), ("温", "溫"),
    ("湾", "灣"), ("湿", "濕"), ("溃", "潰"), ("溅", "濺"), ("滚", "滾"),
    ("滞", "滯"), ("满", "滿"), ("滤", "濾"), ("滥", "濫"), ("滨", "濱"),
    ("滩", "灘"), ("漂", "漂"), ("漏", "漏"), ("漫", "漫"), ("潘", "潘"),
    ("潜", "潛"), ("潴", "瀦"), ("澜", "瀾"), ("濑", "瀨"), ("濒", "瀕"),
    ("灭", "滅"), ("灵", "靈"), ("灾", "災"), ("灿", "燦"), ("炉", "爐"),
    ("炖", "燉"), ("炜", "煒"), ("点", "點"), ("炼", "煉"), ("炽", "熾"),
    ("烁", "爍"), ("烂", "爛"), ("烃", "烴"), ("烛", "燭"), ("烟", "煙"),
    ("烦", "煩"), ("烧", "燒"), ("烨", "燁"), ("烫", "燙"), ("烬", "燼"),
    ("热", "熱"), ("焕", "煥"), ("焖", "燜"), ("爷", "爺"), ("牵", "牽"),
    ("牺", "犧"), ("犊", "犢"), ("状", "狀"), ("犷", "獷"), ("犹", "猶"),
    ("狈", "狽"), ("狞", "獰"), ("独", "獨"), ("狭", "狹"), ("狮", "獅"),
    ("狱", "獄"), ("狰", "猙"), ("狲", "猻"), ("猎", "獵"), ("猕", "獼"),
    ("猪", "豬"), ("猫", "貓"), ("献", "獻"), ("獭", "獺"), ("玛", "瑪"),
    ("玮", "瑋"), ("环", "環"), ("现", "現"), ("玺", "璽"), ("珑", "瓏"),
    ("琼", "瓊"), ("瑶", "瑤"), ("甄", "甄"), ("璃", "璃"), ("瓒", "瓚"),
    ("瓮", "甕"), ("瓯", "甌"), ("瓷", "甆"), ("画", "畫"), ("畅", "暢"),
    ("畴", "疇"), ("疗", "療"), ("疟", "瘧"), ("疡", "瘍"), ("疮", "瘡"),
    ("疯", "瘋"), ("疱", "皰"), ("痈", "癰"), ("痉", "痙"), ("痒", "癢"),
    ("痪", "瘓"), ("痴", "癡"), ("瘸", "瘸"), ("瘘", "瘺"), ("瘪", "癟"),
    ("瘫", "癱"), ("瘾", "癮"), ("癫", "癲"), ("皱", "皺"), ("皲", "皸"),
    ("盏", "盞"), ("盐", "鹽"), ("监", "監"), ("盖", "蓋"), ("盗", "盜"),
    ("盘", "盤"), ("卢", "盧"), ("眍", "瞘"), ("着", "著"), ("睁", "睜"),
    ("睑", "瞼"), ("瞒", "瞞"), ("瞩", "矚"), ("矫", "矯"), ("矶", "磯"),
    ("矾", "礬"), ("矿", "礦"), ("码", "碼"), ("砖", "磚"), ("砚", "硯"),
    ("砜", "砜"), ("砺", "礪"), ("砾", "礫"), ("础", "礎"), ("硕", "碩"),
    ("硖", "硤"), ("硗", "磽"), ("确", "確"), ("碍", "礙"), ("碛", "磧"),
    ("碱", "鹼"), ("礼", "禮"), ("祎", "禕"), ("祯", "禎"), ("祷", "禱"),
    ("祸", "禍"), ("禄", "祿"), ("禅", "禪"), ("离", "離"), ("秃", "禿"),
    ("秆", "稈"), ("积", "積"), ("称", "稱"), ("秽", "穢"), ("税", "稅"),
    ("稣", "穌"), ("稳", "穩"), ("穷", "窮"), ("窃", "竊"), ("窍", "竅"),
    ("窑", "窯"), ("窜", "竄"), ("窝", "窩"), ("窥", "窺"), ("窦", "竇"),
    ("竖", "豎"), ("竞", "競"), ("笋", "筍"), ("笔", "筆"), ("笼", "籠"),
    ("筑", "築"), ("筛", "篩"), ("笺", "箋"), ("筑", "築"), ("筹", "籌"),
    ("签", "簽"), ("简", "簡"), ("箓", "籙"), ("箦", "篋"), ("箩", "籮"),
    ("箫", "簫"), ("篑", "簣"), ("篮", "籃"), ("篱", "籬"), ("籁", "籟"),
    ("粪", "糞"), ("粮", "糧"), ("紧", "緊"), ("纠", "糾"), ("纡", "紆"),
    ("红", "紅"), ("纣", "紂"), ("纤", "纖"), ("约", "約"), ("级", "級"),
    ("纨", "紈"), ("纪", "紀"), ("纬", "緯"), ("纭", "紜"), ("纯", "純"),
    ("纱", "紗"), ("纲", "綱"), ("纳", "納"), ("纵", "縱"), ("纶", "綸"),
    ("纷", "紛"), ("纸", "紙"), ("纹", "紋"), ("纺", "紡"), ("纽", "紐"),
    ("线", "線"), ("绀", "紺"), ("练", "練"), ("组", "組"), ("绅", "紳"),
    ("细", "細"), ("织", "織"), ("终", "終"), ("绊", "絆"), ("绍", "紹"),
    ("绎", "繹"), ("经", "經"), ("绑", "綁"), ("绒", "絨"), ("结", "結"),
    ("绕", "繞"), ("绘", "繪"), ("给", "給"), ("绚", "絢"), ("绛", "絳"),
    ("络", "絡"), ("绝", "絕"), ("绞", "絞"), ("统", "統"), ("绠", "綆"),
    ("绢", "絹"), ("绣", "綉"), ("绥", "綏"), ("继", "繼"), ("绪", "緒"),
    ("绫", "綾"), ("续", "續"), ("绮", "綺"), ("绯", "緋"), ("绰", "綽"),
    ("绳", "繩"), ("维", "維"), ("绵", "綿"), ("绷", "繃"), ("绸", "綢"),
    ("绿", "綠"), ("缀", "綴"), ("缁", "緇"), ("缂", "緙"), ("缄", "緘"),
    ("缅", "緬"), ("缆", "纜"), ("缇", "緹"), ("缉", "緝"), ("缊", "緼"),
    ("缎", "緞"), ("缏", "緶"), ("缐", "線"), ("缑", "緱"), ("缒", "縋"),
    ("缓", "緩"), ("缔", "締"), ("缕", "縷"), ("编", "編"), ("缗", "緡"),
    ("缘", "緣"), ("缙", "縉"), ("缚", "縛"), ("缛", "縟"), ("缜", "縝"),
    ("缝", "縫"), ("缞", "縗"), ("缟", "縞"), ("缠", "纏"), ("缡", "縭"),
    ("缢", "縊"), ("缤", "繽"), ("缥", "縹"), ("缦", "縵"), ("缧", "縲"),
    ("缨", "纓"), ("缩", "縮"), ("缪", "繆"), ("缫", "繅"), ("缭", "繚"),
    ("缮", "繕"), ("缯", "繒"), ("缰", "繮"), ("缱", "繱"), ("缲", "繰"),
    ("缳", "繯"), ("缴", "繳"), ("缵", "纘"), ("网", "網"), ("罗", "羅"),
    ("罚", "罰"), ("罢", "罷"), ("羁", "羈"), ("翘", "翹"), ("耧", "耬"),
    ("耸", "聳"), ("耻", "恥"), ("聂", "聶"), ("聋", "聾"), ("职", "職"),
    ("联", "聯"), ("聩", "聵"), ("聪", "聰"), ("肃", "肅"), ("肠", "腸"),
    ("肤", "膚"), ("肾", "腎"), ("胀", "脹"), ("胁", "脅"), ("胆", "膽"),
    ("胜", "勝"), ("胧", "朧"), ("胪", "臚"), ("胫", "脛"), ("胶", "膠"),
    ("脉", "脈"), ("脍", "膾"), ("脏", "髒"), ("脐", "臍"), ("脑", "腦"),
    ("脓", "膿"), ("脚", "腳"), ("脱", "脫"), ("脸", "臉"), ("腊", "臘"),
    ("腌", "醃"), ("腻", "膩"), ("腾", "騰"), ("膑", "臏"), ("舆", "輿"),
    ("舰", "艦"), ("舱", "艙"), ("艰", "艱"), ("艳", "豔"), ("艺", "藝"),
    ("节", "節"), ("芜", "蕪"), ("芦", "蘆"), ("苇", "葦"), ("苏", "蘇"),
    ("苹", "蘋"), ("茎", "莖"), ("茧", "繭"), ("荆", "荊"), ("荐", "薦"),
    ("荚", "莢"), ("荟", "薈"), ("荠", "薺"), ("荡", "蕩"), ("荣", "榮"),
    ("荤", "葷"), ("荧", "熒"), ("荫", "蔭"), ("药", "藥"), ("莅", "蒞"),
    ("莱", "萊"), ("莲", "蓮"), ("获", "獲"), ("莹", "瑩"), ("莺", "鶯"),
    ("莼", "蓴"), ("萝", "蘿"), ("萤", "螢"), ("营", "營"), ("萦", "縈"),
    ("萧", "蕭"), ("萨", "薩"), ("葱", "蔥"), ("蒋", "蔣"), ("蒌", "蔞"),
    ("蓝", "藍"), ("蓟", "薊"), ("蔺", "藺"), ("蔼", "藹"), ("蕲", "蘄"),
    ("蕴", "蘊"), ("薮", "藪"), ("藓", "蘚"), ("虏", "虜"), ("虑", "慮"),
    ("虚", "虛"), ("虫", "蟲"), ("虽", "雖"), ("虾", "蝦"), ("蚀", "蝕"),
    ("蚁", "蟻"), ("蚂", "螞"), ("蚕", "蠶"), ("蛊", "蠱"), ("蛎", "蠣"),
    ("蛮", "蠻"), ("蛰", "蟄"), ("蜗", "蝸"), ("蜡", "蠟"), ("蝇", "蠅"),
    ("蝉", "蟬"), ("蝎", "蠍"), ("蝗", "蝗"), ("蝼", "螻"), ("蝾", "蠑"),
    ("蝙", "蝙"), ("蝉", "蟬"), ("蝎", "蠍"), ("蝗", "蝗"), ("蝼", "螻"),
    ("蝾", "蠑"), ("螀", "螿"), ("螨", "蟎"), ("衅", "釁"), ("衔", "銜"),
    ("补", "補"), ("衬", "襯"), ("衮", "袞"), ("袄", "襖"), ("袅", "嫋"),
    ("袜", "襪"), ("袭", "襲"), ("装", "裝"), ("裆", "襠"), ("裤", "褲"),
    ("裥", "襇"), ("褛", "襤"), ("见", "見"), ("观", "觀"), ("规", "規"),
    ("觅", "覓"), ("视", "視"), ("览", "覽"), ("觉", "覺"), ("觇", "覘"),
    ("觊", "覬"), ("觋", "覡"), ("觌", "覿"), ("觎", "覦"), ("觏", "覯"),
    ("觐", "覲"), ("觑", "覷"), ("触", "觸"), ("觯", "觶"), ("誉", "譽"),
    ("誊", "謄"), ("计", "計"), ("订", "訂"), ("认", "認"), ("讥", "譏"),
    ("讨", "討"), ("让", "讓"), ("讪", "訕"), ("讫", "訖"), ("训", "訓"),
    ("议", "議"), ("讯", "訊"), ("记", "記"), ("讲", "講"), ("讳", "諱"),
    ("讴", "謳"), ("讷", "訥"), ("许", "許"), ("讹", "訛"), ("论", "論"),
    ("讼", "訟"), ("讽", "諷"), ("设", "設"), ("访", "訪"), ("诀", "訣"),
    ("证", "證"), ("诂", "詁"), ("诃", "訶"), ("评", "評"), ("诅", "詛"),
    ("识", "識"), ("诈", "詐"), ("诉", "訴"), ("诊", "診"), ("词", "詞"),
    ("诏", "詔"), ("译", "譯"), ("诒", "詒"), ("诓", "誆"), ("试", "試"),
    ("诗", "詩"), ("诘", "詰"), ("诚", "誠"), ("诛", "誅"), ("话", "話"),
    ("诞", "誕"), ("诟", "詬"), ("诠", "詮"), ("诡", "詭"), ("询", "詢"),
    ("诣", "詣"), ("该", "該"), ("详", "詳"), ("诧", "詫"), ("诨", "諢"),
    ("诫", "誡"), ("诬", "誣"), ("语", "語"), ("诮", "誚"), ("误", "誤"),
    ("诰", "誥"), ("诱", "誘"), ("诲", "誨"), ("诳", "誑"), ("说", "說"),
    ("诵", "誦"), ("请", "請"), ("诸", "諸"), ("诹", "諏"), ("诺", "諾"),
    ("读", "讀"), ("诽", "誹"), ("课", "課"), ("谁", "誰"), ("谂", "諗"),
    ("调", "調"), ("谅", "諒"), ("谆", "諄"), ("谈", "談"), ("谊", "誼"),
    ("谋", "謀"), ("谎", "謊"), ("谏", "諫"), ("谐", "諧"), ("谒", "謁"),
    ("谓", "謂"), ("谔", "諤"), ("谕", "諭"), ("谗", "讒"), ("谙", "諳"),
    ("谚", "諺"), ("谛", "諦"), ("谜", "謎"), ("谟", "謨"), ("谠", "讜"),
    ("谢", "謝"), ("谣", "謠"), ("谤", "謗"), ("谥", "諡"), ("谦", "謙"),
    ("谧", "謐"), ("谨", "謹"), ("谩", "謾"), ("谪", "謫"), ("谬", "謬"),
    ("谭", "譚"), ("谮", "譖"), ("谯", "譙"), ("谱", "譜"), ("谲", "譎"),
    ("谳", "讞"), ("谴", "譴"), ("谵", "譫"), ("谶", "讖"), ("贝", "貝"),
    ("贞", "貞"), ("负", "負"), ("贡", "貢"), ("财", "財"), ("责", "責"),
    ("贤", "賢"), ("败", "敗"), ("账", "賬"), ("货", "貨"), ("质", "質"),
    ("贩", "販"), ("贪", "貪"), ("贫", "貧"), ("贬", "貶"), ("购", "購"),
    ("贮", "貯"), ("贯", "貫"), ("贱", "賤"), ("贲", "賁"), ("贴", "貼"),
    ("贵", "貴"), ("贷", "貸"), ("贸", "貿"), ("费", "費"), ("贺", "賀"),
    ("贻", "貽"), ("贼", "賊"), ("贾", "賈"), ("贿", "賄"), ("赁", "賃"),
    ("赂", "賂"), ("赃", "贓"), ("资", "資"), ("赅", "賅"), ("赈", "賑"),
    ("赊", "賒"), ("赋", "賦"), ("赌", "賭"), ("赍", "齎"), ("赎", "贖"),
    ("赏", "賞"), ("赐", "賜"), ("赔", "賠"), ("赖", "賴"), ("赘", "贅"),
    ("赚", "賺"), ("赛", "賽"), ("赝", "贗"), ("赞", "贊"), ("赠", "贈"),
    ("赡", "贍"), ("赢", "贏"), ("赣", "贛"), ("赵", "趙"), ("赶", "趕"),
    ("趋", "趨"), ("跃", "躍"), ("践", "踐"), ("跷", "蹺"), ("跸", "蹕"),
    ("跻", "躋"), ("踊", "踴"), ("踌", "躊"), ("踪", "蹤"), ("蹑", "躡"),
    ("蹒", "蹣"), ("车", "車"), ("轧", "軋"), ("轨", "軌"), ("轩", "軒"),
    ("转", "轉"), ("轮", "輪"), ("软", "軟"), ("轰", "轟"), ("轴", "軸"),
    ("轶", "軼"), ("轻", "輕"), ("载", "載"), ("较", "較"), ("辅", "輔"),
    ("辆", "輛"), ("辈", "輩"), ("辉", "輝"), ("辐", "輻"), ("辑", "輯"),
    ("输", "輸"), ("辔", "轡"), ("辕", "轅"), ("辖", "轄"), ("辞", "辭"),
    ("辩", "辯"), ("辫", "辮"), ("边", "邊"), ("辽", "遼"), ("达", "達"),
    ("迁", "遷"), ("迈", "邁"), ("运", "運"), ("还", "還"), ("进", "進"),
    ("远", "遠"), ("违", "違"), ("连", "連"), ("迟", "遲"), ("迹", "跡"),
    ("适", "適"), ("选", "選"), ("逊", "遜"), ("递", "遞"), ("逻", "邏"),
    ("遗", "遺"), ("遥", "遙"), ("邓", "鄧"), ("邝", "鄺"), ("邮", "郵"),
    ("邹", "鄒"), ("邻", "鄰"), ("郏", "郟"), ("郑", "鄭"), ("郓", "鄆"),
    ("郸", "鄲"), ("酝", "醞"), ("酱", "醬"), ("释", "釋"), ("鉴", "鑒"),
    ("针", "針"), ("钉", "釘"), ("钊", "釗"), ("钓", "釣"), ("钙", "鈣"),
    ("钠", "鈉"), ("钢", "鋼"), ("钥", "鑰"), ("钦", "欽"), ("钧", "鈞"),
    ("钨", "鎢"), ("钩", "鉤"), ("钮", "鈕"), ("钱", "錢"), ("钳", "鉗"),
    ("钴", "鈷"), ("钵", "缽"), ("钾", "鉀"), ("铀", "鈾"), ("铁", "鐵"),
    ("铂", "鉑"), ("铃", "鈴"), ("铅", "鉛"), ("铆", "鉚"), ("铎", "鐸"),
    ("铐", "銬"), ("铛", "鐺"), ("铜", "銅"), ("铝", "鋁"), ("铠", "鎧"),
    ("铨", "銓"), ("铬", "鉻"), ("铭", "銘"), ("铮", "錚"), ("铯", "銫"),
    ("铰", "鉸"), ("铱", "銥"), ("铲", "鏟"), ("银", "銀"), ("铸", "鑄"),
    ("铺", "鋪"), ("链", "鏈"), ("铿", "鏗"), ("销", "銷"), ("锁", "鎖"),
    ("锂", "鋰"), ("锄", "鋤"), ("锅", "鍋"), ("锈", "鏽"), ("锋", "鋒"),
    ("锌", "鋅"), ("错", "錯"), ("锚", "錨"), ("锟", "錕"), ("锡", "錫"),
    ("锢", "錮"), ("锣", "鑼"), ("锤", "錘"), ("锥", "錐"), ("锦", "錦"),
    ("键", "鍵"), ("锯", "鋸"), ("锰", "錳"), ("锱", "錙"), ("锻", "鍛"),
    ("镀", "鍍"), ("镁", "鎂"), ("镂", "鏤"), ("镇", "鎮"), ("镊", "鑷"),
    ("镌", "鐫"), ("镍", "鎳"), ("镐", "鎬"), ("镑", "鎊"), ("镖", "鏢"),
    ("镗", "鏜"), ("镜", "鏡"), ("镝", "鏑"), ("镰", "鐮"), ("镱", "鐿"),
    ("长", "長"), ("门", "門"), ("闪", "閃"), ("闭", "閉"), ("问", "問"),
    ("闯", "闖"), ("闰", "閏"), ("闲", "閒"), ("间", "間"), ("闵", "閔"),
    ("闷", "悶"), ("闸", "閘"), ("闹", "鬧"), ("闻", "聞"), ("闽", "閩"),
    ("阀", "閥"), ("阁", "閣"), ("阅", "閱"), ("阆", "閬"), ("阈", "閾"),
    ("阉", "閹"), ("阊", "閶"), ("阎", "閻"), ("阐", "闡"), ("阑", "闌"),
    ("阔", "闊"), ("阕", "闋"), ("阖", "闔"), ("阗", "闐"), ("阙", "闕"),
    ("阚", "闞"), ("队", "隊"), ("阳", "陽"), ("阴", "陰"), ("阵", "陣"),
    ("阶", "階"), ("际", "際"), ("陆", "陸"), ("陇", "隴"), ("陈", "陳"),
    ("陕", "陝"), ("陨", "隕"), ("险", "險"), ("随", "隨"), ("隐", "隱"),
    ("隶", "隸"), ("难", "難"), ("雏", "雛"), ("雳", "靂"), ("雾", "霧"),
    ("霁", "霽"), ("霉", "黴"), ("霭", "靄"), ("靓", "靚"), ("静", "靜"),
    ("靥", "靨"), ("韦", "韋"), ("韧", "韌"), ("韩", "韓"), ("韬", "韜"),
    ("韵", "韻"), ("页", "頁"), ("顶", "頂"), ("顷", "頃"), ("项", "項"),
    ("顺", "順"), ("须", "須"), ("顾", "顧"), ("顿", "頓"), ("颁", "頒"),
    ("颂", "頌"), ("预", "預"), ("颅", "顱"), ("领", "領"), ("颇", "頗"),
    ("颈", "頸"), ("颊", "頰"), ("颌", "頜"), ("颐", "頤"), ("频", "頻"),
    ("颓", "頹"), ("颔", "頷"), ("颖", "穎"), ("颗", "顆"), ("题", "題"),
    ("颢", "顥"), ("颤", "顫"), ("颧", "顴"), ("风", "風"), ("飏", "颺"),
    ("飒", "颯"), ("飓", "颶"), ("飙", "飆"), ("飞", "飛"), ("饭", "飯"),
    ("饮", "飲"), ("饰", "飾"), ("饱", "飽"), ("饲", "飼"), ("馆", "館"),
    ("饼", "餅"), ("饿", "餓"), ("馁", "餒"), ("馒", "饅"), ("馔", "饌"),
    ("马", "馬"), ("驭", "馭"), ("驮", "馱"), ("驯", "馴"), ("驰", "馳"),
    ("驱", "驅"), ("驳", "駁"), ("驴", "驢"), ("驶", "駛"), ("驷", "駟"),
    ("驹", "駒"), ("驻", "駐"), ("驼", "駝"), ("驾", "駕"), ("驿", "驛"),
    ("骀", "駘"), ("骁", "驍"), ("骂", "罵"), ("骈", "駢"), ("骊", "驪"),
    ("骋", "騁"), ("验", "驗"), ("骏", "駿"), ("骑", "騎"), ("骒", "騍"),
    ("骓", "騅"), ("骖", "驂"), ("骗", "騙"), ("骘", "騭"), ("骛", "騖"),
    ("骜", "驁"), ("骝", "騮"), ("骞", "騫"), ("骠", "驃"), ("骡", "騾"),
    ("骢", "驄"), ("骤", "驟"), ("骥", "驥"), ("骧", "驤"), ("髅", "髏"),
    ("髋", "髖"), ("鬓", "鬢"), ("魇", "魘"), ("鱼", "魚"), ("鱿", "魷"),
    ("鲁", "魯"), ("鲂", "魴"), ("鲈", "鱸"), ("鲋", "鮒"), ("鲍", "鮑"),
    ("鲎", "鱟"), ("鲜", "鮮"), ("鲞", "鯗"), ("鲟", "鱘"), ("鲤", "鯉"),
    ("鲨", "鯊"), ("鲫", "鯽"), ("鲱", "鯡"), ("鲲", "鯤"), ("鲳", "鯧"),
    ("鲴", "鯝"), ("鲵", "鯢"), ("鲶", "鯰"), ("鲷", "鯛"), ("鲸", "鯨"),
    ("鲹", "鯵"), ("鳆", "鰒"), ("鳇", "鰉"), ("鳊", "鯿"), ("鳋", "鰠"),
    ("鳌", "鼇"), ("鳍", "鰭"), ("鳏", "鰥"), ("鳐", "鰩"), ("鳓", "鰳"),
    ("鳔", "鰾"), ("鳕", "鱈"), ("鳖", "鱉"), ("鳗", "鰻"), ("鳙", "鰼"),
    ("鳜", "鱔"), ("鳝", "鱔"), ("鳞", "鱗"), ("鳟", "鱒"), ("鳢", "鱧"),
    ("鸟", "鳥"), ("鸠", "鳩"), ("鸡", "雞"), ("鸢", "鳶"), ("鸣", "鳴"),
    ("鸥", "鷗"), ("鸦", "鴉"), ("鸨", "鴇"), ("鸪", "鴣"), ("鸫", "鶇"),
    ("鸭", "鴨"), ("鸯", "鴦"), ("鸰", "鴒"), ("鸱", "鴟"), ("鸳", "鴛"),
    ("鸵", "鴕"), ("鸶", "鷥"), ("鸷", "鷙"), ("鸽", "鴿"), ("鸾", "鸞"),
    ("鸿", "鴻"), ("鹁", "鵓"), ("鹂", "鸝"), ("鹃", "鵑"), ("鹄", "鵠"),
    ("鹅", "鵝"), ("鹆", "鵒"), ("鹊", "鵲"), ("鹌", "鵪"), ("鹏", "鵬"),
    ("鹑", "鶉"), ("鹒", "鶊"), ("鹔", "鷞"), ("鹖", "鶡"), ("鹗", "鶚"),
    ("鹘", "鶻"), ("鹙", "鶖"), ("鹛", "鶥"), ("鹜", "鶩"), ("鹞", "鷂"),
    ("鹟", "鶲"), ("鹠", "鶹"), ("鹡", "鶺"), ("鹢", "鷁"), ("鹣", "鶼"),
    ("鹤", "鶴"), ("鹦鹉", "鸚"), ("鹧", "鷓"), ("鹨", "鷚"), ("鹩", "鷯"),
    ("鹪", "鷦"), ("鹫", "鷲"), ("鹬", "鷸"), ("鹭", "鷺"), ("鹰", "鷹"),
    ("鹳", "鸛"), ("鹾", "鹺"), ("麦", "麥"), ("麸", "麩"), ("黄", "黃"),
    ("黉", "黌"), ("黡", "黶"), ("鼋", "黿"), ("鼍", "鼉"), ("齐", "齊"),
    ("齑", "齏"), ("齿", "齒"), ("龀", "齔"), ("龂", "齗"), ("龃", "齟"),
    ("龄", "齡"), ("龅", "齙"), ("龇", "齜"), ("龈", "齦"), ("龉", "齬"),
    ("龊", "齪"), ("龋", "齲"), ("龌", "齷"), ("龙", "龍"), ("龚", "龔"),
    ("龛", "龕"), ("龟", "龜"), ("尝", "嘗"), ("准", "準"), ("钟", "鐘"),
    ("恒", "恆"),
];

// Build lookup maps
thread_local! {
    static S2T_MAP: RefCell<HashMap<String, String>> = RefCell::new(build_map(S2T_DATA));
    static T2S_MAP: RefCell<HashMap<String, String>> = RefCell::new(build_reverse_map(S2T_DATA));
}

fn build_map(data: &[(&str, &str)]) -> HashMap<String, String> {
    data.iter().map(|&(s, t)| (s.to_string(), t.to_string())).collect()
}

fn build_reverse_map(data: &[(&str, &str)]) -> HashMap<String, String> {
    data.iter().map(|&(s, t)| (t.to_string(), s.to_string())).collect()
}

static JAVA_VM: OnceCell<Mutex<Option<JavaVM>>> = OnceCell::new();

fn ensure_vm() {
    JAVA_VM.get_or_init(|| Mutex::new(None));
}

// ===== Trans: s2t / t2s =====

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeInit(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    ensure_vm();
    if let Ok(vm) = env.get_java_vm() {
        let _ = JAVA_VM.get().unwrap().lock().replace(vm);
    }
    env.new_string("").unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_utils_RustUtil_nativeInit(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    ensure_vm();
    if let Ok(vm) = env.get_java_vm() {
        let _ = JAVA_VM.get().unwrap().lock().replace(vm);
    }
    env.new_string("").unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeS2t(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    ensure_vm();
    let s = jstring_to_str(&mut env, &text);
    to_jstring(&mut env, &s2t_impl(&s)).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_utils_RustUtil_nativeS2t(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    ensure_vm();
    let s = jstring_to_str(&mut env, &text);
    to_jstring(&mut env, &s2t_impl(&s)).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeT2s(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    ensure_vm();
    let s = jstring_to_str(&mut env, &text);
    to_jstring(&mut env, &t2s_impl(&s)).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_utils_RustUtil_nativeT2s(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    ensure_vm();
    let s = jstring_to_str(&mut env, &text);
    to_jstring(&mut env, &t2s_impl(&s)).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

// ===== Hex / MD5 =====

fn s2t_impl(text: &str) -> String {
    if text.is_empty() { return text.to_string(); }
    let mut result = String::with_capacity(text.len());
    let mut i = 0;
    while i < text.len() {
        if let Some(ch) = text[i..].chars().next() {
            let ch_str = ch.to_string();
            if let Some(replacement) = S2T_MAP.with(|m| m.borrow().get(&ch_str).cloned()) {
                result.push_str(&replacement);
                i += ch.len_utf8();
                continue;
            }
            result.push(ch);
            i += ch.len_utf8();
        } else {
            break;
        }
    }
    result
}

fn t2s_impl(text: &str) -> String {
    if text.is_empty() { return text.to_string(); }
    let mut result = String::with_capacity(text.len());
    let mut i = 0;
    while i < text.len() {
        if let Some(ch) = text[i..].chars().next() {
            let ch_str = ch.to_string();
            if let Some(replacement) = T2S_MAP.with(|m| m.borrow().get(&ch_str).cloned()) {
                result.push_str(&replacement);
                i += ch.len_utf8();
                continue;
            }
            result.push(ch);
            i += ch.len_utf8();
        } else {
            break;
        }
    }
    result
}

// ===== Util: hex2byte, md5 =====

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeHex2byte(
    mut env: JNIEnv,
    _class: JClass,
    hex: JString,
) -> jbyteArray {
    ensure_vm();
    let s = jstring_to_str(&mut env, &hex);
    if s.len() % 2 != 0 { return std::ptr::null_mut(); }
    let bytes: Vec<u8> = (0..s.len()).step_by(2)
        .filter_map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok()).collect();
    env.byte_array_from_slice(&bytes).map(|arr| arr.into_raw()).unwrap_or_else(|_| std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_utils_RustUtil_nativeHex2byte(
    mut env: JNIEnv,
    _class: JClass,
    hex: JString,
) -> jbyteArray {
    ensure_vm();
    let s = jstring_to_str(&mut env, &hex);
    if s.len() % 2 != 0 { return std::ptr::null_mut(); }
    let bytes: Vec<u8> = (0..s.len()).step_by(2)
        .filter_map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok()).collect();
    env.byte_array_from_slice(&bytes).map(|arr| arr.into_raw()).unwrap_or_else(|_| std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeMd5(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    ensure_vm();
    let s = jstring_to_str(&mut env, &text);
    if s.is_empty() { return env.new_string("").unwrap().into_raw(); }
    let result = compute(s.as_bytes());
    let hex = result.0.iter().fold(String::with_capacity(32), |mut acc, b| {
        let _ = write!(acc, "{:02x}", b);
        acc
    });
    to_jstring(&mut env, &hex).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_utils_RustUtil_nativeMd5(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    ensure_vm();
    let s = jstring_to_str(&mut env, &text);
    if s.is_empty() { return env.new_string("").unwrap().into_raw(); }
    let result = compute(s.as_bytes());
    let hex = result.0.iter().fold(String::with_capacity(32), |mut acc, b| {
        let _ = write!(acc, "{:02x}", b);
        acc
    });
    to_jstring(&mut env, &hex).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

// ===== Decoder: AES/CBC 解密 =====

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeCbcDecrypt(
    mut env: JNIEnv,
    _class: JClass,
    data: JString,
) -> jstring {
    ensure_vm();
    let s = jstring_to_str(&mut env, &data);
    let lower = s.to_lowercase();
    let key = match extract_key(&lower) { Some(k) => k, None => return env.new_string("").unwrap().into_raw() };
    let iv = extract_iv(&lower);
    let ct_hex = extract_ciphertext(&s);

    let key_bytes = match hex::decode(&key) { Ok(b) => b, Err(_) => return env.new_string("").unwrap().into_raw() };
    let iv_bytes = match hex::decode(&iv) { Ok(b) => b, Err(_) => return env.new_string("").unwrap().into_raw() };
    let ciphertext = match hex::decode(&ct_hex) { Ok(b) => b, Err(_) => return env.new_string("").unwrap().into_raw() };

    let cipher = match Aes128::new_from_slice(&key_bytes) { Ok(c) => c, Err(_) => return env.new_string("").unwrap().into_raw() };
    let mut blocks = ciphertext.clone();

    let mut prev = [0u8; 16];
    prev.copy_from_slice(&iv_bytes);

    for chunk in blocks.chunks_exact_mut(16) {
        let mut block_arr: [u8; 16] = [0u8; 16];
        block_arr.copy_from_slice(chunk);
        let mut block = GenericArray::from(block_arr);
        cipher.decrypt_block(&mut block);
        for i in 0..16 {
            chunk[i] = block[i] ^ prev[i];
        }
        prev.copy_from_slice(chunk);
    }

    let pad = match blocks.last() { Some(&b) => b, None => return env.new_string("").unwrap().into_raw() };
    if pad == 0 || pad > 16 { return env.new_string("").unwrap().into_raw(); }
    let pad_len = pad as usize;
    blocks.truncate(blocks.len() - pad_len);

    let result = String::from_utf8_lossy(&blocks).into_owned();
    to_jstring(&mut env, &result).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_utils_RustUtil_nativeCbcDecrypt(
    mut env: JNIEnv,
    _class: JClass,
    data: JString,
) -> jstring {
    ensure_vm();
    let s = jstring_to_str(&mut env, &data);
    let lower = s.to_lowercase();
    let key = match extract_key(&lower) { Some(k) => k, None => return env.new_string("").unwrap().into_raw() };
    let iv = extract_iv(&lower);
    let ct_hex = extract_ciphertext(&s);

    let key_bytes = match hex::decode(&key) { Ok(b) => b, Err(_) => return env.new_string("").unwrap().into_raw() };
    let iv_bytes = match hex::decode(&iv) { Ok(b) => b, Err(_) => return env.new_string("").unwrap().into_raw() };
    let ciphertext = match hex::decode(&ct_hex) { Ok(b) => b, Err(_) => return env.new_string("").unwrap().into_raw() };

    let cipher = match Aes128::new_from_slice(&key_bytes) { Ok(c) => c, Err(_) => return env.new_string("").unwrap().into_raw() };
    let mut blocks = ciphertext.clone();

    let mut prev = [0u8; 16];
    prev.copy_from_slice(&iv_bytes);

    for chunk in blocks.chunks_exact_mut(16) {
        let mut block_arr: [u8; 16] = [0u8; 16];
        block_arr.copy_from_slice(chunk);
        let mut block = GenericArray::from(block_arr);
        cipher.decrypt_block(&mut block);
        for i in 0..16 {
            chunk[i] = block[i] ^ prev[i];
        }
        prev.copy_from_slice(chunk);
    }

    let pad = match blocks.last() { Some(&b) => b, None => return env.new_string("").unwrap().into_raw() };
    if pad == 0 || pad > 16 { return env.new_string("").unwrap().into_raw(); }
    let pad_len = pad as usize;
    blocks.truncate(blocks.len() - pad_len);

    let result = String::from_utf8_lossy(&blocks).into_owned();
    to_jstring(&mut env, &result).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

fn extract_key(s: &str) -> Option<String> {
    let start = s.find("$#")? + 2;
    let end = s[start..].find("#$")? + start;
    let key = &s[start..end];
    Some(format!("{:0>16}", key))
}

fn extract_iv(s: &str) -> String {
    let start = s.len().saturating_sub(13);
    format!("{:0>16}", &s[start..])
}

fn extract_ciphertext(s: &str) -> String {
    let marker = "2324";
    let start = match s.find(marker) { Some(i) => i + 4, None => return String::new() };
    let end = s.len().saturating_sub(26);
    s[start..end].to_string()
}

// ===== Auth: Digest 认证 =====

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeDigest(
    mut env: JNIEnv,
    _class: JClass,
    user_info: JString,
    header: JString,
    method: JString,
    uri: JString,
) -> jstring {
    ensure_vm();
    let ui = jstring_to_str(&mut env, &user_info);
    let hdr = jstring_to_str(&mut env, &header);
    let mth = jstring_to_str(&mut env, &method);
    let uri = jstring_to_str(&mut env, &uri);
    if ui.is_empty() || hdr.is_empty() || mth.is_empty() || uri.is_empty() {
        return env.new_string("").unwrap().into_raw();
    }

    let params = parse_digest_header(&hdr[7..]);
    let parts: Vec<&str> = ui.splitn(2, ':').collect();
    let username = parts[0];
    let password = parts.get(1).copied().unwrap_or("");

    let realm = params.get("realm").map(|s| s.as_str()).unwrap_or("");
    let nonce = params.get("nonce").map(|s| s.as_str()).unwrap_or("");
    let opaque = params.get("opaque").map(|s| s.as_str());
    let qop = select_qop(params.get("qop").map(|s| s.as_str()).unwrap_or(""));
    let nc = "00000001";
    let cnonce = Uuid::new_v4().to_string().replace('-', "");

    let ha1 = md5_hex(format!("{}:{}:{}", username, realm, password));
    let ha2 = md5_hex(format!("{}:{}", mth, uri));
    let response = if qop.is_empty() {
        md5_hex(format!("{}:{}:{}", ha1, nonce, ha2))
    } else {
        md5_hex(format!("{}:{}:{}:{}:{}:{}", ha1, nonce, nc, cnonce, qop, ha2))
    };

    let mut fields = vec![
        format!("username=\"{}\"", username),
        format!("realm=\"{}\"", realm),
        format!("nonce=\"{}\"", nonce),
        format!("uri=\"{}\"", uri),
    ];
    if !qop.is_empty() {
        fields.push(format!("cnonce=\"{}\"", cnonce));
        fields.push(format!("nc={}", nc));
        fields.push(format!("qop={}", qop));
    }
    fields.push(format!("response=\"{}\"", response));
    if let Some(op) = opaque { fields.push(format!("opaque=\"{}\"", op)); }

    let result = format!("Digest {}", fields.join(", "));
    to_jstring(&mut env, &result).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_utils_RustUtil_nativeDigest(
    mut env: JNIEnv,
    _class: JClass,
    user_info: JString,
    header: JString,
    method: JString,
    uri: JString,
) -> jstring {
    ensure_vm();
    let ui = jstring_to_str(&mut env, &user_info);
    let hdr = jstring_to_str(&mut env, &header);
    let mth = jstring_to_str(&mut env, &method);
    let uri = jstring_to_str(&mut env, &uri);
    if ui.is_empty() || hdr.is_empty() || mth.is_empty() || uri.is_empty() {
        return env.new_string("").unwrap().into_raw();
    }

    let params = parse_digest_header(&hdr[7..]);
    let parts: Vec<&str> = ui.splitn(2, ':').collect();
    let username = parts[0];
    let password = parts.get(1).copied().unwrap_or("");

    let realm = params.get("realm").map(|s| s.as_str()).unwrap_or("");
    let nonce = params.get("nonce").map(|s| s.as_str()).unwrap_or("");
    let opaque = params.get("opaque").map(|s| s.as_str());
    let qop = select_qop(params.get("qop").map(|s| s.as_str()).unwrap_or(""));
    let nc = "00000001";
    let cnonce = Uuid::new_v4().to_string().replace('-', "");

    let ha1 = md5_hex(format!("{}:{}:{}", username, realm, password));
    let ha2 = md5_hex(format!("{}:{}", mth, uri));
    let response = if qop.is_empty() {
        md5_hex(format!("{}:{}:{}", ha1, nonce, ha2))
    } else {
        md5_hex(format!("{}:{}:{}:{}:{}:{}", ha1, nonce, nc, cnonce, qop, ha2))
    };

    let mut fields = vec![
        format!("username=\"{}\"", username),
        format!("realm=\"{}\"", realm),
        format!("nonce=\"{}\"", nonce),
        format!("uri=\"{}\"", uri),
    ];
    if !qop.is_empty() {
        fields.push(format!("cnonce=\"{}\"", cnonce));
        fields.push(format!("nc={}", nc));
        fields.push(format!("qop={}", qop));
    }
    fields.push(format!("response=\"{}\"", response));
    if let Some(op) = opaque { fields.push(format!("opaque=\"{}\"", op)); }

    let result = format!("Digest {}", fields.join(", "));
    to_jstring(&mut env, &result).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

fn parse_digest_header(header: &str) -> HashMap<String, String> {
    let re = Regex::new(r#"(\w+)=(?:"([^"]*)"|([^,\s"]+))"#).unwrap();
    let mut params = HashMap::new();
    for cap in re.captures_iter(header) {
        let key = cap[1].to_string();
        let val = cap.get(2).map(|m| m.as_str()).or_else(|| cap.get(3).map(|m| m.as_str())).unwrap_or("").trim().to_string();
        params.insert(key, val);
    }
    params
}

fn select_qop(qop: &str) -> &str {
    if qop.is_empty() { return ""; }
    for opt in qop.split(',') {
        if opt.trim().eq_ignore_ascii_case("auth") { return "auth"; }
    }
    ""
}

fn md5_hex(input: String) -> String {
    let r = compute(input.as_bytes());
    r.0.iter().fold(String::with_capacity(32), |mut acc, b| {
        let _ = write!(acc, "{:02x}", b);
        acc
    })
}

// ===== QueryUtil =====

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeQueryGet(
    mut env: JNIEnv,
    _class: JClass,
    query: JString,
    key: JString,
) -> jstring {
    ensure_vm();
    let q = jstring_to_str(&mut env, &query);
    let k = jstring_to_str(&mut env, &key);
    if q.is_empty() || k.is_empty() { return env.new_string("").unwrap().into_raw(); }
    let prefix = format!("{}=", k);
    for pair in q.split('&') {
        if let Some(rest) = pair.strip_prefix(&prefix) {
            return url_decode_jstring(&mut env, rest);
        }
    }
    env.new_string("").unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_github_catvod_utils_RustUtil_nativeQueryGet(
    mut env: JNIEnv,
    _class: JClass,
    query: JString,
    key: JString,
) -> jstring {
    ensure_vm();
    let q = jstring_to_str(&mut env, &query);
    let k = jstring_to_str(&mut env, &key);
    if q.is_empty() || k.is_empty() { return env.new_string("").unwrap().into_raw(); }
    let prefix = format!("{}=", k);
    for pair in q.split('&') {
        if let Some(rest) = pair.strip_prefix(&prefix) {
            return url_decode_jstring(&mut env, rest);
        }
    }
    env.new_string("").unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeQueryToMap(
    mut env: JNIEnv,
    _class: JClass,
    query: JString,
) -> jstring {
    ensure_vm();
    let q = jstring_to_str(&mut env, &query);
    if q.is_empty() {
        return to_jstring(&mut env, "{}").unwrap_or_else(|_| env.new_string("").unwrap().into_raw());
    }
    let mut map: HashMap<String, String> = HashMap::new();
    for pair in q.split('&') {
        if let Some(idx) = pair.find('=') {
            map.insert(pair[..idx].to_string(), url_decode_str(&pair[idx + 1..]));
        }
    }
    let json = serde_json::to_string(&map).unwrap_or_default();
    to_jstring(&mut env, &json).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

fn url_decode_str(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let h1 = chars.next().and_then(|c| c.to_digit(16));
            let h2 = chars.next().and_then(|c| c.to_digit(16));
            if let (Some(h1), Some(h2)) = (h1, h2) {
                result.push(char::from_u32((h1 << 4) | h2).unwrap_or('?'));
                continue;
            }
        } else if ch == '+' { result.push(' '); continue; }
        result.push(ch);
    }
    result
}

fn url_decode_jstring(env: &mut JNIEnv, value: &str) -> jstring {
    to_jstring(env, &url_decode_str(value)).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

// ===== UriUtil: RFC 3986 =====

const SCHEME_COLON: usize = 0;
const PATH: usize = 1;
const QUERY: usize = 2;
const FRAGMENT: usize = 3;

#[no_mangle]
pub extern "system" fn Java_com_fongmi_android_tv_util_RustUtil_nativeResolveUri(
    mut env: JNIEnv,
    _class: JClass,
    base_uri: JString,
    ref_uri: JString,
) -> jstring {
    ensure_vm();
    let base = jstring_to_str(&mut env, &base_uri);
    let reference = jstring_to_str(&mut env, &ref_uri);
    let result = resolve_uri_v2(&base, &reference);
    to_jstring(&mut env, &result).unwrap_or_else(|_| env.new_string("").unwrap().into_raw())
}

fn resolve_uri_v2(base: &str, reference: &str) -> String {
    if base.is_empty() && reference.is_empty() { return String::new(); }
    let bi = get_uri_indices(base);
    let ri = get_uri_indices(reference);

    if ri[SCHEME_COLON] != 0 && ri[SCHEME_COLON] < ri[QUERY] {
        return remove_dot_segments_final(reference, ri[PATH], ri[QUERY]);
    }
    if ri[FRAGMENT] == 0 {
        return format!("{}{}", &base[..bi[FRAGMENT]], reference);
    }
    if ri[QUERY] == 0 {
        return format!("{}{}", &base[..bi[QUERY]], reference);
    }
    if ri[PATH] != 0 {
        let base_limit = bi[SCHEME_COLON] + 1;
        let combined = format!("{}{}", &base[..base_limit], reference);
        let ci = get_uri_indices(&combined);
        return remove_dot_segments_final(&combined, ci[PATH], ci[QUERY]);
    }
    if reference.len() > ri[PATH] && reference.as_bytes().get(ri[PATH]).copied() == Some(b'/') {
        let combined = format!("{}{}", &base[..bi[PATH]], reference);
        let ci = get_uri_indices(&combined);
        return remove_dot_segments_final(&combined, ci[PATH], bi[PATH] + ri[QUERY]);
    }
    if bi[SCHEME_COLON] + 2 < bi[PATH] && bi[PATH] == bi[QUERY] {
        let combined = format!("{}/{}", &base[..bi[PATH]], reference);
        let ci = get_uri_indices(&combined);
        return remove_dot_segments_final(&combined, ci[PATH], bi[PATH] + ri[QUERY] + 1);
    }
    let last_slash = match base[..bi[QUERY]].rfind('/') {
        Some(pos) if pos + 1 <= bi[QUERY] => pos + 1,
        _ => bi[PATH],
    };
    let combined = format!("{}{}", &base[..last_slash], reference);
    remove_dot_segments_final(&combined, bi[PATH], last_slash + ri[QUERY])
}

fn get_uri_indices(u: &str) -> [usize; 4] {
    let mut idx = [0usize; 4];
    if u.is_empty() {
        idx[SCHEME_COLON] = usize::MAX;
        return idx;
    }
    let len = u.len();
    let frag = u.find('#').unwrap_or(len);
    let mut query = u.find('?').unwrap_or(frag);
    if query > frag { query = frag; }
    let mut scheme_limit = u.find('/').unwrap_or(query);
    if scheme_limit > query { scheme_limit = query; }
    let scheme = u.find(':').unwrap_or(scheme_limit + 1);
    let sc = if scheme <= scheme_limit { scheme } else { usize::MAX };
    idx[SCHEME_COLON] = sc;

    if sc != usize::MAX {
        let has_authority = sc + 2 < query
            && u.len() > sc + 2
            && u.as_bytes().get(sc + 1).copied() == Some(b'/')
            && u.as_bytes().get(sc + 2).copied() == Some(b'/');
        if has_authority {
            let after_auth = &u[sc + 3..];
            let path = after_auth.find('/').map(|p| sc + 3 + p).unwrap_or(query);
            idx[PATH] = path;
        } else {
            idx[PATH] = sc + 1;
        }
    } else {
        idx[PATH] = 0;
    }
    idx[QUERY] = query;
    idx[FRAGMENT] = frag;
    idx
}

fn remove_dot_segments_final(uri: &str, offset: usize, limit: usize) -> String {
    if offset >= limit { return uri.to_string(); }
    let mut buf = uri.as_bytes().to_vec();
    let mut lim = limit;

    let mut segment_start = offset;
    let mut i = offset;
    while i <= lim {
        let next_seg_start;
        if i == lim {
            next_seg_start = i;
        } else if i < buf.len() && buf[i] == b'/' {
            next_seg_start = i + 1;
        } else {
            i += 1;
            continue;
        }
        if i == segment_start + 1 && segment_start < buf.len() && buf[segment_start] == b'.' {
            buf.drain(segment_start..next_seg_start.min(buf.len()));
            let drained = next_seg_start - segment_start;
            lim -= drained;
            i = segment_start;
        } else if i == segment_start + 2
            && segment_start + 1 < buf.len()
            && buf[segment_start] == b'.'
            && buf[segment_start + 1] == b'.'
        {
            let search_end = segment_start.saturating_sub(2);
            let prev = buf[..=search_end]
                .iter()
                .rposition(|&b| b == b'/')
                .map(|p| p + 1)
                .unwrap_or(0);
            let remove_from = prev.max(offset);
            let drain_end = next_seg_start.min(buf.len());
            buf.drain(remove_from..drain_end);
            let drained = drain_end - remove_from;
            lim -= drained;
            segment_start = prev;
            i = prev;
        } else {
            i += 1;
            segment_start = i;
        }
    }
    String::from_utf8_lossy(&buf[..lim.min(buf.len())]).into_owned()
}

// ===== Helpers =====

fn jstring_to_str(env: &mut JNIEnv, js: &JString) -> String {
    match env.get_string(js) { Ok(s) => s.into(), Err(_) => String::new() }
}

fn to_jstring(env: &mut JNIEnv, value: &str) -> Result<jstring, jni::errors::Error> {
    Ok(env.new_string(value)?.into_raw())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s2t() {
        let r = s2t_impl("中国");
        assert!(r.contains('國'), "{}", r);
    }

    #[test]
    fn test_t2s() {
        let r = t2s_impl("中國");
        assert!(r.contains('国'), "{}", r);
    }

    #[test]
    fn test_md5() {
        assert_eq!(md5_hex("hello".to_string()), "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(url_decode_str("hello%20world"), "hello world");
        assert_eq!(url_decode_str("a+b"), "a b");
    }

    #[test]
    fn test_resolve() {
        assert_eq!(resolve_uri_v2("http://example.com/path", "http://other.com/page"), "http://other.com/page");
        assert_eq!(resolve_uri_v2("http://example.com/a/b/c", "d/e"), "http://example.com/a/b/d/e");
        assert_eq!(resolve_uri_v2("http://example.com/a/b", "./c"), "http://example.com/a/c");
        assert_eq!(resolve_uri_v2("http://example.com/a/b", "../c"), "http://example.com/c");
        assert_eq!(resolve_uri_v2("http://example.com", "/path"), "http://example.com/path");
        assert_eq!(resolve_uri_v2("http://example.com/path", "#frag"), "http://example.com/path#frag");
        assert_eq!(resolve_uri_v2("http://example.com/path?q", "?p"), "http://example.com/path?p");
    }
}
