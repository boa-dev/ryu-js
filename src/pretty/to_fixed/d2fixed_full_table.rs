pub(crate) const TABLE_SIZE: usize = 3;

pub(crate) const POW10_OFFSET: [u8; TABLE_SIZE] = [0, 2, 5];

#[rustfmt::skip]
pub(crate) const POW10_SPLIT: [[u64; 3]; 8] = [
    [                   1,  72057594037927936,           0], // 0
    [  699646928636035157,           72057594,           0], // 1
    [                   1,                  0,         256], // 2
    [11902091922964236229,      4722366482869,           0], // 3
    [ 6760415703743915872,               4722,           0], // 4
    [                   1,                  0,    16777216], // 5

    [13369850649504950658, 309485009821345068,           0], // 5 (+1 length)
    [15151142278969419334,          309485009,           0], // 5 (+2 length)
];

pub(crate) const TABLE_SIZE_2: usize = 27;
pub(crate) const ADDITIONAL_BITS_2: usize = 120;

#[rustfmt::skip]
pub(crate) const POW10_OFFSET_2: [u16; TABLE_SIZE_2 + 1] = [
    /*  0 */ 0,
    /*  1 */ 2,
    /* 02 */ 6, 
    /* 03 */ 12, 
    /* 04 */ 20, 
    /* 05 */ 29, 
    /* 06 */ 40, 
    /* 07 */ 52, 
    /* 08 */ 66, 
    /* 09 */ 80, 
    /* 10 */ 95, 
    /* 11 */ 112, 
    /* 12 */ 130, 
    /* 13 */ 150, 
    /* 14 */ 170, 
    /* 15 */ 192, 
    /* 16 */ 215, 
    /* 17 */ 240, 
    /* 18 */ 265, 
    /* 19 */ 292, 
    /* 20 */ 320, 
    /* 21 */ 350,
    /* 22 */ 381, 
    /* 23 */ 413, 
    /* 24 */ 446, 
    /* 25 */ 480,
    /* 26 */ 516, 
    /* 27 */ 552, 
    // /* 28 */ 590, 
];

#[rustfmt::skip]
pub(crate) const MIN_BLOCK_2: [u8; TABLE_SIZE_2] = [
    /*  0 */ 0,
    /*  1 */ 0,
    /*  2 */ 0,
    /*  3 */ 0,
    /*  4 */ 0,
    /*  5 */ 0,
    /*  6 */ 1,
    /*  7 */ 1,
    /*  8 */ 2,
    /*  9 */ 3,
    /* 10 */ 3,
    /* 11 */ 4,
    /* 12 */ 4,
    /* 13 */ 5,
    /* 14 */ 5,
    /* 15 */ 6,
    /* 16 */ 6,
    /* 17 */ 7,
    /* 18 */ 7,
    /* 19 */ 8,
    /* 20 */ 8,
    /* 21 */ 9,
    /* 22 */ 9,
    /* 23 */ 10,
    /* 24 */ 11,
    /* 25 */ 11, 
    /* 26 */ 12,
];

#[rustfmt::skip]
pub(crate) const POW10_SPLIT_2: [[u64; 3]; 552 + 12] = [
    [0, 0, 3906250],
    [0, 0, 202000000000],
    [0, 11153727427136454656, 59],
    [0, 7205759403792793600, 59604644775],
    [0, 0, 167390625000],
    [0, 0, 232000000000],
    [0, 16777216000000000, 0],
    [0, 12945425605062557696, 909494],
    [0, 4388757836872548352, 182701772928],
    [0, 1152921504606846976, 128237915039],
    [0, 0, 159062500000],
    [0, 0, 160000000000],
    [0, 256000000000, 0],
    [0, 16192327041775828992, 13],
    [0, 15024075324038053888, 13877787807],
    [0, 5449091666327633920, 159814456755],
    [0, 2494994193563254784, 179295395851],
    [0, 4611686018427387904, 11135253906],
    [0, 0, 146250000000],
    [0, 0, 128000000000],
    [0, 3906250, 0],
    [0, 3906250000000000, 0],
    [0, 4368439412768899072, 211758],
    [0, 1563676642168012800, 46236813575],
    [0, 11532349341402398720, 7084767080],
    [0, 9048364970084925440, 104625169910],
    [0, 16609275425742389248, 246490512847],
    [0, 0, 207900390625],
    [0, 0, 225000000000],
    [11153727427136454656, 59, 0],
    [7205759403792793600, 59604644775, 0],
    [0, 4264412554261970152, 3],
    [0, 14485570586272534528, 3231174267],
    [0, 17827675094632103936, 123785264354],
    [0, 7347197909193981952, 226966440203],
    [0, 13677404030777688064, 11398292396],
    [0, 3810326759732150272, 172741453558],
    [0, 9943947977234055168, 246206558227],
    [0, 0, 19539062500],
    [0, 0, 228000000000],
    [12945425605062557696, 909494, 0],
    [4388757836872548352, 909494701772928, 0],
    [1152921504606846976, 14878706826214591391, 49303],
    [0, 4387341015746028192, 151806576313],
    [0, 651726680428265472, 185237838233],
    [0, 2570638187944738816, 153035330174],
    [0, 7419175577111756800, 126139354575],
    [0, 17299322326264840192, 207402194313],
    [0, 7990511638862102528, 137937798142],
    [0, 16717361816799281152, 254433166503],
    [0, 0, 167906250000],
    [0, 0, 16000000000],
    [16192327041775828992, 13, 0],
    [15024075324038053888, 13877787807, 0],
    [5449091666327633920, 13877787807814456755, 0],
    [2494994193563254784, 9707857417284919307, 752316384],
    [4611686018427387904, 1844515466944871826, 224526264005],
    [0, 15167599819856275072, 197099991383],
    [0, 14830185305589481472, 87822237233],
    [0, 6163721531743535104, 49803945956],
    [0, 14122847407012052992, 228334136013],
    [0, 335491783960035328, 205765601092],
    [0, 941252322120433664, 68018187046],
    [0, 11529215046068469760, 38051025390],
    [0, 0, 238625000000],
    [0, 0, 64000000000],
    [4368439412768899072, 211758, 0],
    [1563676642168012800, 211758236813575, 0],
    [11532349341402398720, 8061591463141767016, 11479],
    [9048364970084925440, 16628725344207857142, 215437019748],
    [16609275425742389248, 3555541870038531535, 100901445007],
    [0, 18316647450161853665, 143192746310],
    [0, 16709574568378075648, 70992947447],
    [0, 7696022835795591168, 247905827852],
    [0, 16664449640376041472, 12417202233],
    [0, 3109186955116544000, 57903381625],
    [0, 10515518101817131008, 121168549362],
    [0, 9961962375743537152, 242570047378],
    [0, 9223372036854775808, 146540039062],
    [0, 0, 150500000000],
    [14485570586272534528, 3231174267, 0],
    [17827675094632103936, 3231174267785264354, 0],
    [7347197909193981952, 748977172262750475, 175162308],
    [13677404030777688064, 15965033457315095468, 196040602133],
    [3810326759732150272, 16809402149066729206, 21865466197],
    [9943947977234055168, 7563769067065700371, 85911239516],
    [0, 13550322810840051428, 92410032742],
    [0, 8663209637545764864, 102734564471],
    [0, 8969247575312957440, 119469633535],
    [0, 6193172891660451840, 255486223885],
    [0, 3427954273864908800, 13335732575],
    [0, 10058367555266936832, 95185829773],
    [0, 13907115649320091648, 141545265197],
    [0, 0, 45753906250],
    [0, 0, 74000000000],
    [14878706826214591391, 49303, 0],
    [4387341015746028192, 49303806576313, 0],
    [651726680428265472, 14106411361315920281, 2672],
    [2570638187944738816, 3609034283485221502, 112764710092],
    [7419175577111756800, 9896072247338192335, 204195646140],
    [17299322326264840192, 8889095178479228297, 188536467151],
    [7990511638862102528, 3631796911038383102, 207481878815],
    [16717361816799281152, 898318840772166823, 31196880105],
    [0, 17293677953982795024, 233048697961],
    [0, 7353628266884669440, 105937492160],
    [0, 2404693032470315008, 192398640987],
    [0, 9191155893041889280, 91130358670],
    [0, 6353946855033798656, 142498253559],
    [0, 3767824038248841216, 247344448149],
    [0, 7205759403792793600, 149204254150],
    [0, 0, 198390625000],
    [0, 0, 232000000000],
    [9707857417284919307, 752316384, 0],
    [1844515466944871826, 752316384526264005, 0],
    [15167599819856275072, 17063068157692817751, 40783152],
    [14830185305589481472, 5385330256507239985, 48924990778],
    [6163721531743535104, 3373050282752075748, 58291939338],
    [14122847407012052992, 4116064001262906061, 10182853422],
    [335491783960035328, 11306582046748043076, 46223132276],
    [941252322120433664, 17035410946089626406, 116612931040],
    [11529215046068469760, 15618595715183448558, 224923491477],
    [0, 5141740092277295680, 149846685770],
    [0, 16973644291514990592, 74278734288],
    [0, 14625255268443750400, 208920143100],
    [0, 14021170507320131584, 252792836676],
    [0, 4451355232865091584, 68760089176],
    [0, 12891553933348044800, 88241308450],
    [0, 1152921504606846976, 34698852539],
    [0, 0, 187062500000],
    [0, 0, 160000000000],
    [8061591463141767016, 11479, 0],
    [16628725344207857142, 11479437019748, 0],
    [3555541870038531535, 5562205901560339855, 622],
    [18316647450161853665, 2106077949367544134, 110301527786],
    [16709574568378075648, 7496855998374373623, 234114170714],
    [7696022835795591168, 229183437194837004, 90406405378],
    [16664449640376041472, 465169186276472889, 2012424059],
    [3109186955116544000, 2152980561625316473, 123025216872],
    [10515518101817131008, 2059790725449340402, 104116713310],
    [9961962375743537152, 17891190926410198930, 94111661478],
    [9223372036854775808, 9930696175609809814, 166969883403],
    [0, 7276914261609005312, 11538344118],
    [0, 10539762974036983808, 182394482312],
    [0, 12851089458992250880, 136571361695],
    [0, 9449311677678878720, 159696658955],
    [0, 8699564697382289408, 11512248212],
    [0, 4224376450473525248, 148471604347],
    [0, 4611686018427387904, 123229003906],
    [0, 0, 130250000000],
    [0, 0, 128000000000],
    [748977172262750475, 175162308, 0],
    [15965033457315095468, 175162308040602133, 0],
    [16809402149066729206, 13756840147955779925, 9495567],
    [7563769067065700371, 13788447602092505948, 15745759798],
    [13550322810840051428, 4972540435632173670, 54747473242],
    [8663209637545764864, 2844874687533091959, 90269561957],
    [8969247575312957440, 15377573779532804095, 101154220965],
    [6193172891660451840, 17824715805091194381, 165833619944],
    [3427954273864908800, 18277569135638159711, 232966279779],
    [10058367555266936832, 4254645803379752845, 99990829008],
    [13907115649320091648, 2933643244178200621, 208230644811],
    [0, 17188148801879487562, 75159033118],
    [0, 11069762501163246592, 30931771413],
    [0, 11676570643941818368, 21600093027],
    [0, 17840016768744030208, 99632988162],
    [0, 16463817321652158464, 2967109246],
    [0, 6954191143357644800, 126892505325],
    [0, 5080060379673919488, 237376987457],
    [0, 0, 65275390625],
    [0, 0, 161000000000],
    [14106411361315920281, 2672, 0],
    [3609034283485221502, 2672764710092, 0],
    [9896072247338192335, 16433563478020213436, 144],
    [8889095178479228297, 4194750497955655375, 144890865261],
    [3631796911038383102, 2691539602252904735, 109227397880],
    [898318840772166823, 3775467271962795241, 248145908654],
    [17293677953982795024, 16980212613224918121, 174204668490],
    [7353628266884669440, 4172857038337333440, 74920499170],
    [2404693032470315008, 5936867627376461659, 226226211033],
    [9191155893041889280, 17856837443266866062, 217321838238],
    [6353946855033798656, 8956297047799810807, 158968021097],
    [3767824038248841216, 15356974049716912789, 105485521835],
    [7205759403792793600, 6923608913322982854, 171832503231],
    [0, 4855902993563955944, 191375329591],
    [0, 13835893222288330752, 55263239028],
    [0, 9114973913760137216, 116750045274],
    [0, 17937099003422310400, 90494123725],
    [0, 7007960010734960640, 205972372085],
    [0, 7683422439270776832, 117379902273],
    [0, 720575940379279360, 65416519165],
    [0, 0, 253039062500],
    [0, 0, 228000000000],
    [17063068157692817751, 40783152, 0],
    [5385330256507239985, 40783152924990778, 0],
    [3373050282752075748, 2768933352715741194, 2210859],
    [4116064001262906061, 15201941611824153390, 43150104177],
    [11306582046748043076, 1418128541727000180, 113824098906],
    [17035410946089626406, 5353350204565757408, 90076876902],
    [15618595715183448558, 1721001680354286741, 102290205696],
    [5141740092277295680, 637631411660453962, 93295688],
    [16973644291514990592, 1630012588870568400, 72034566068],
    [14625255268443750400, 9253063571656828156, 180088363159],
    [14021170507320131584, 6029146854993203780, 151501609581],
    [4451355232865091584, 16987401965352759896, 109326840705],
    [12891553933348044800, 14499131620542087970, 129920888905],
    [1152921504606846976, 1978417255298660539, 73785999500],
    [0, 5790079354402454176, 140107250214],
    [0, 13748918935842078720, 38313880830],
    [0, 18047438014740692992, 254745330388],
    [0, 3116889656839372800, 212978353575],
    [0, 15995952446606147584, 167168966926],
    [0, 12530140063251562496, 14867142319],
    [0, 16717361816799281152, 175679260253],
    [0, 0, 93906250000],
    [0, 0, 16000000000],
    [5562205901560339855, 622, 0],
    [2106077949367544134, 622301527786, 0],
    [7496855998374373623, 13558973353698967386, 33],
    [229183437194837004, 6228991722850501890, 33735033418],
    [465169186276472889, 16886831391703377787, 74337674317],
    [2152980561625316473, 1181713637872883048, 77915436964],
    [2059790725449340402, 12393932434925221726, 164064060824],
    [17891190926410198930, 10684799845419711910, 152671876423],
    [9930696175609809814, 4590318792215640843, 71579224160],
    [7276914261609005312, 6383712187366189238, 96248841680],
    [10539762974036983808, 1904270214927675016, 208346061731],
    [12851089458992250880, 3711506775113308575, 163103230695],
    [9449311677678878720, 8091219444738793995, 231201201185],
    [8699564697382289408, 39436684991068052, 33438625885],
    [4224376450473525248, 18025182908196512891, 93002137866],
    [4611686018427387904, 7853924592034603138, 10977147123],
    [0, 4815749283615688320, 243425762105],
    [0, 14242399906544287744, 57261062291],
    [0, 76242322576113664, 147772082046],
    [0, 10858088421377703936, 126004133104],
    [0, 14293835879041466368, 240588618152],
    [0, 12182236992037191680, 168774870395],
    [0, 11529215046068469760, 123660400390],
    [0, 0, 6625000000],
    [0, 0, 64000000000],
    [13756840147955779925, 9495567, 0],
    [13788447602092505948, 9495567745759798, 0],
    [4972540435632173670, 14000097438505379162, 514755],
    [2844874687533091959, 16451062686452429925, 195758946802],
    [15377573779532804095, 4009347599785716645, 242891813895],
    [17824715805091194381, 16544162347546196456, 7217347168],
    [18277569135638159711, 17674258299745817187, 96896860837],
    [4254645803379752845, 5215238411201214416, 165958123462],
    [2933643244178200621, 14253990228345322571, 198282718640],
    [17188148801879487562, 11214836553940194590, 176772710358],
    [11069762501163246592, 14620711348380590101, 214607957507],
    [11676570643941818368, 6638710787931587427, 3792590350],
    [17840016768744030208, 17320000343692853250, 14359885232],
    [16463817321652158464, 75147386268843646, 176938919100],
    [6954191143357644800, 17938801582125480173, 188004073747],
    [5080060379673919488, 6573358613626446145, 19972464382],
    [0, 8688505427903736481, 254356342484],
    [0, 539870168696556032, 212471004823],
    [0, 9002861336394465280, 151029266420],
    [0, 17989846818158018560, 244488046090],
    [0, 2700938287723315200, 10975231550],
    [0, 17800090499088908288, 62146418157],
    [0, 8809040871136690176, 237964944839],
    [0, 9223372036854775808, 199477539062],
    [0, 0, 246500000000],
    [16433563478020213436, 144, 0],
    [4194750497955655375, 144890865261, 0],
    [2691539602252904735, 15763656745260536568, 7],
    [3775467271962795241, 8787336846248645550, 7854549544],
    [16980212613224918121, 17584084447880694346, 40476362484],
    [4172857038337333440, 18041672551129683938, 244953235127],
    [5936867627376461659, 14025886302294509785, 183978041028],
    [17856837443266866062, 18430498103283160734, 196760344819],
    [8956297047799810807, 3292348826238025833, 243999119304],
    [15356974049716912789, 9211721212658275243, 200178478587],
    [6923608913322982854, 10233245872666307519, 251499368407],
    [4855902993563955944, 6200995035623311671, 215554745370],
    [13835893222288330752, 8480542380570450804, 26336156614],
    [9114973913760137216, 11870363864499900506, 198459731123],
    [17937099003422310400, 9301051379839581901, 179643493714],
    [7007960010734960640, 11456694803569638005, 82504211005],
    [7683422439270776832, 14327208890643983169, 61621068669],
    [720575940379279360, 4510081789599866365, 125776679550],
    [0, 13255356976020303332, 126244492023],
    [0, 9658806854127314944, 247718574341],
    [0, 13708435528809971712, 5523604968],
    [0, 1580190652103131136, 232743135779],
    [0, 16557336970347413504, 35085662306],
    [0, 12751520132434493440, 98897575035],
    [0, 9295429630892703744, 123691261291],
    [0, 0, 107503906250],
    [0, 0, 202000000000],
    [2768933352715741194, 2210859, 0],
    [15201941611824153390, 2210859150104177, 0],
    [1418128541727000180, 16872870088062921306, 119850],
    [5353350204565757408, 5112979788807802982, 42914680120],
    [1721001680354286741, 13742728082020150272, 56277175189],
    [637631411660453962, 2217110934613627976, 149744994782],
    [1630012588870568400, 11021433940188610484, 222120189824],
    [9253063571656828156, 1713669895470733463, 128597473131],
    [6029146854993203780, 3313382510572018285, 107092898231],
    [16987401965352759896, 14976595232784069505, 183179618825],
    [14499131620542087970, 7213172372862496841, 9811882854],
    [1978417255298660539, 15836474542502248588, 102391026857],
    [5790079354402454176, 3221099285878340134, 169858497005],
    [13748918935842078720, 3265814602578095358, 237174616142],
    [18047438014740692992, 6502528252282225364, 78177040164],
    [3116889656839372800, 16392476834556790183, 36352502762],
    [15995952446606147584, 15167629413417091342, 234888637949],
    [12530140063251562496, 1366763272626280111, 253822238838],
    [16717361816799281152, 8720523635169216093, 118074092385],
    [0, 9649171375767398672, 97472740533],
    [0, 7647980704001073152, 181523082628],
    [0, 13286434495608651776, 132414597864],
    [0, 4358271637167013888, 232720259057],
    [0, 15954987941890097152, 241236262378],
    [0, 7911135695429697536, 234864921629],
    [0, 7205759403792793600, 29428863525],
    [0, 0, 37390625000],
    [0, 0, 232000000000],
    [13558973353698967386, 33, 0],
    [6228991722850501890, 33735033418, 0],
    [16886831391703377787, 15288289344628122701, 1],
    [1181713637872883048, 952589339068938148, 1828779826],
    [12393932434925221726, 10058155040190817688, 50051639971],
    [10684799845419711910, 5322725640026584391, 163545253677],
    [4590318792215640843, 2269982385930389600, 45288545535],
    [6383712187366189238, 13216683679976310224, 255123055991],
    [1904270214927675016, 17417440642083494819, 119716477857],
    [3711506775113308575, 3029180749090900711, 161944201349],
    [8091219444738793995, 8315443826261908513, 133164212217],
    [39436684991068052, 1488962797247197277, 249450781113],
    [18025182908196512891, 18009099634999034122, 185080716834],
    [7853924592034603138, 8092455412807497971, 34976275247],
    [4815749283615688320, 17808458047236758329, 47438692886],
    [14242399906544287744, 3164591817527425171, 22965398445],
    [76242322576113664, 3314036340472350590, 173171552866],
    [10858088421377703936, 33234902404332784, 98179654270],
    [14293835879041466368, 12349284717857274280, 126001801667],
    [12182236992037191680, 18209607903013119355, 195669456065],
    [11529215046068469760, 7891549145984268038, 193987144822],
    [0, 7703609897518594624, 118427801736],
    [0, 6336912652634587136, 136417613529],
    [0, 4461621834659397632, 217343524723],
    [0, 5484660635557953536, 115241865004],
    [0, 15142619273265938432, 44297324048],
    [0, 12170977992968765440, 16820883035],
    [0, 1152921504606846976, 91659790039],
    [0, 0, 215062500000],
    [0, 0, 160000000000],
    [14000097438505379162, 514755, 0],
    [16451062686452429925, 514755758946802, 0],
    [4009347599785716645, 17812314011563521031, 27904],
    [16544162347546196456, 7684138864490314336, 965607477],
    [17674258299745817187, 9740522787420029605, 53416558002],
    [5215238411201214416, 6701109407732989894, 178528034798],
    [14253990228345322571, 16534886227502443952, 238363267868],
    [11214836553940194590, 8908667306968317910, 28896357978],
    [14620711348380590101, 7531472173477105155, 90482939822],
    [6638710787931587427, 11527371604834801166, 174408281924],
    [17320000343692853250, 15688593496691078576, 68624900066],
    [75147386268843646, 11394944804253312188, 226850480357],
    [17938801582125480173, 11182279880854372627, 229617721195],
    [6573358613626446145, 150579373068361470, 107606192607],
    [8688505427903736481, 3147220002440857300, 223008162924],
    [539870168696556032, 3630514817795505815, 108170611138],
    [9002861336394465280, 11708796588334233588, 194196810602],
    [17989846818158018560, 16844495466426369546, 106634735134],
    [2700938287723315200, 17636655472325475902, 30913141928],
    [17800090499088908288, 17038926655686645229, 168956085008],
    [8809040871136690176, 15602838456783529415, 16923682064],
    [9223372036854775808, 10869815869248876790, 16845831567],
    [0, 18407124180939800832, 143589253898],
    [0, 5705018517251293184, 10997852201],
    [0, 9660452258743058432, 41309269673],
    [0, 5646292272224927744, 169523694166],
    [0, 7410409304047484928, 86306086117],
    [0, 5953758707383795712, 229401719093],
    [0, 4611686018427387904, 53322753906],
    [0, 0, 114250000000],
    [0, 0, 128000000000],
    [15763656745260536568, 7, 0],
    [8787336846248645550, 7854549544, 0],
    [17584084447880694346, 7854549544476362484, 0],
    [18041672551129683938, 15035424419724983, 425795984],
    [14025886302294509785, 18280822466032836292, 144000815071],
    [18430498103283160734, 11524250747302615283, 223991005371],
    [3292348826238025833, 15212285943691810760, 187624730884],
    [9211721212658275243, 7951804027551297019, 4824659673],
    [10233245872666307519, 1706416229965221847, 217431068160],
    [6200995035623311671, 3406023111930700826, 92505009],
    [8480542380570450804, 16132696204133391302, 177184640882],
    [11870363864499900506, 11593846688794356915, 114874555213],
    [9301051379839581901, 6875759884161133906, 77628503688],
    [11456694803569638005, 3593593325323835965, 136372735690],
    [14327208890643983169, 9542049733257388925, 202194809084],
    [4510081789599866365, 9926551925937787518, 252517275552],
    [13255356976020303332, 3128491553219547895, 160538119458],
    [9658806854127314944, 17158408656931354885, 34169595866],
    [13708435528809971712, 2065169543154992616, 218930159197],
    [1580190652103131136, 4832622393556232739, 93111953065],
    [16557336970347413504, 16505930714733656162, 169261976984],
    [12751520132434493440, 18270988073492888699, 152894788296],
    [9295429630892703744, 2525111411519708523, 200990472248],
    [0, 16728989342518570442, 56136886563],
    [0, 7974052022039438336, 35906880329],
    [0, 5356554962386550784, 73432274226],
    [0, 6693869495028547584, 50290379426],
    [0, 8157517147199766528, 162362875392],
    [0, 12065776720423157760, 442219890],
    [0, 11997589407315001344, 114654087066],
    [0, 0, 154650390625],
    [0, 0, 97000000000],
    [16872870088062921306, 119850, 0],
    [5112979788807802982, 119850914680120, 0],
    [13742728082020150272, 2418433229320326037, 6497],
    [2217110934613627976, 1143911773589293534, 97131103528],
    [11021433940188610484, 9276183703610924928, 40062011581],
    [1713669895470733463, 3532180128827684715, 189502862926],
    [3313382510572018285, 8563997501322031543, 78191479868],
    [14976595232784069505, 14843890409658460681, 60464255234],
    [7213172372862496841, 9489417861634552678, 2804688911],
    [15836474542502248588, 1113198223322322089, 15514422373],
    [3221099285878340134, 11190777557146597869, 101060346596],
    [3265814602578095358, 17764553645932638286, 228606653266],
    [6502528252282225364, 14900777150991234852, 82963018382],
    [16392476834556790183, 17364899863357893610, 142807772747],
    [15167629413417091342, 15537570181590167037, 75941353107],
    [1366763272626280111, 5558052627121307766, 147842293367],
    [8720523635169216093, 12095241565795232609, 119301302636],
    [9649171375767398672, 2187936505958366389, 108655684359],
    [7647980704001073152, 12009203621325860228, 7118608275],
    [13286434495608651776, 14814842834750302952, 147651020232],
    [4358271637167013888, 5965296499605198833, 200803114239],
    [15954987941890097152, 4051026394962148842, 255323379371],
    [7911135695429697536, 16799526299141688349, 171219606580],
    [7205759403792793600, 9460214166646215205, 52910704145],
    [0, 10750736995029068008, 17512839237],
    [0, 5377963045376430080, 69582798620],
    [0, 15996910350253424640, 28291539960],
    [0, 13651157529655246848, 248867194247],
    [0, 9771305410219737088, 135740030732],
    [0, 12709439623416250368, 12529703527],
    [0, 9943947977234055168, 103688980102],
    [0, 0, 134539062500],
    [0, 0, 228000000000],
    [952589339068938148, 1828779826, 0],
    [10058155040190817688, 1828779826051639971, 0],
    [5322725640026584391, 371564423966525229, 99138353],
    [2269982385930389600, 14464859121514339583, 49020142547],
    [13216683679976310224, 3913119023023056247, 211784141584],
    [17417440642083494819, 5493396321716566945, 16212130607],
    [3029180749090900711, 5837454566818211973, 47297797611],
    [8315443826261908513, 2886670683193253881, 235316449046],
    [1488962797247197277, 5504823105587173817, 22156486731],
    [18009099634999034122, 9431834277334851106, 75298417058],
    [8092455412807497971, 12921661346456247087, 162511300760],
    [17808458047236758329, 3643076516404724246, 152700484665],
    [3164591817527425171, 12559396953196866477, 57197491573],
    [3314036340472350590, 1626880974916825698, 117680846273],
    [33234902404332784, 6806994170946429566, 193088193394],
    [12349284717857274280, 7596631230206896579, 114369007893],
    [18209607903013119355, 3100480253729502401, 21411814204],
    [7891549145984268038, 6310570748781063286, 60168077371],
    [7703609897518594624, 14251867077375744136, 59342096725],
    [6336912652634587136, 6701165793751570137, 85772595262],
    [4461621834659397632, 10856833140463959923, 62363270925],
    [5484660635557953536, 15867563727561248556, 13588550103],
    [15142619273265938432, 5048961008671491600, 215860182353],
    [12170977992968765440, 13278183119599849051, 81273704724],
    [1152921504606846976, 4547591784941053655, 20719811749],
    [0, 11815437715887182496, 165246525444],
    [0, 398495392178782208, 4640516162],
    [0, 9154841240825495552, 66021602478],
    [0, 1902683298245640192, 174496284938],
    [0, 5081900962138816512, 10103144668],
    [0, 3234710432358858752, 220275490403],
    [0, 16717361816799281152, 99175354003],
    [0, 0, 147906250000],
    [0, 0, 16000000000],
    [17812314011563521031, 27904, 0],
    [7684138864490314336, 27904965607477, 0],
    [9740522787420029605, 13488568028574514610, 1512],
    [6701109407732989894, 275784718433886190, 232731216738],
    [16534886227502443952, 10020568880357102364, 98014950319],
    [8908667306968317910, 8876397213146246746, 175543216127],
    [7531472173477105155, 2155905919114811310, 255481190457],
    [11527371604834801166, 1087100407155601220, 57116871894],
    [15688593496691078576, 2903498381705011170, 214058931831],
    [11394944804253312188, 12223476257006657765, 119157398962],
    [11182279880854372627, 12148657163736735595, 178662635975],
    [150579373068361470, 8951241323311673823, 199658580024],
    [3147220002440857300, 8463862715901576300, 56485247764],
    [3630514817795505815, 3873401978748963266, 20458826917],
    [11708796588334233588, 248364795947002730, 165209977542],
    [16844495466426369546, 10454378025404001822, 198013463882],
    [17636655472325475902, 6574176865628265640, 74566732968],
    [17038926655686645229, 16703315293848336, 168356386842],
    [15602838456783529415, 9896033222450013456, 26000905488],
    [10869815869248876790, 17311376269334085007, 16536465035],
    [18407124180939800832, 18378511316495639306, 139938451587],
    [5705018517251293184, 15120796393727584297, 131996301094],
    [9660452258743058432, 18253447805740347049, 38819700014],
    [5646292272224927744, 5842497225601731158, 46989521388],
    [7410409304047484928, 4369968404176723173, 236316722409],
    [5953758707383795712, 16142207253674488117, 233236896461],
    [4611686018427387904, 12124259227391928178, 205875070808],
    [0, 13019483264566077056, 88657257409],
    [0, 74901376448135168, 193705787602],
    [0, 13897060093813325824, 210004060411],
    [0, 4495486210810052608, 251753361137],
    [0, 14885496280087265280, 241243700795],
    [0, 4976477588244398080, 59806944370],
    [0, 11529215046068469760, 114269775390],
    [0, 0, 30625000000],
    [0, 0, 64000000000],
    [15035424419724983, 425795984, 0],
    [18280822466032836292, 425795984000815071, 0],
    [11524250747302615283, 10043594327130472635, 23082446],
    [15212285943691810760, 8336034337032909060, 206544464339],
    [7951804027551297019, 16717215784895280857, 211451897326],
    [1706416229965221847, 10968831263951212032, 238906242083],
    [3406023111930700826, 5536629379734406065, 35594621534],
    [16132696204133391302, 1618806894932332402, 94300141280],
    [11593846688794356915, 11363331325254998861, 224087755697],
    [6875759884161133906, 8775167772751754888, 177616007425],
    [3593593325323835965, 2898202945316114122, 1475702798],
    [9542049733257388925, 8868842714495185148, 14157111896],
    [9926551925937787518, 17052094667531999136, 88480780926],
    [3128491553219547895, 3658615537031138594, 126924395904],
    [17158408656931354885, 12486952437987190746, 128198333945],
    [2065169543154992616, 912079238520577629, 249676919048],
    [4832622393556232739, 10960072898031888041, 8049443914],
    [16505930714733656162, 6129550094334741912, 74594146742],
    [18270988073492888699, 7965724516573729480, 182332283576],
    [2525111411519708523, 5801761178810791992, 184431822791],
    [16728989342518570442, 13197466483098446115, 199314514103],
    [7974052022039438336, 11326268638393107273, 183715436091],
    [5356554962386550784, 3597339351794947378, 59613998253],
    [6693869495028547584, 353880726151383714, 173195012157],
    [8157517147199766528, 11154818162602073600, 61019183912],
    [12065776720423157760, 5141043976157511026, 40604703904],
    [11997589407315001344, 7188225141808859034, 160278696552],
    [0, 13894168943295705185, 104389674465],
    [0, 12176538069834828288, 225753204407],
    [0, 7994239409235165184, 183660091451],
    [0, 13707777025480065024, 59433368586],
    [0, 10120227247676719104, 10743100081],
    [0, 7358494763030413312, 177548618618],
    [0, 7656119366529843200, 122398904800],
    [0, 9223372036854775808, 224415039062],
    [0, 0, 86500000000],
    [2418433229320326037, 6497, 0],
    [1143911773589293534, 6497131103528, 0],
    [9276183703610924928, 3877189582299842749, 352],
    [3532180128827684715, 7625565791857948238, 96210182868],
    [8563997501322031543, 16568435163612007484, 212413382749],
    [14843890409658460681, 17592071940521808130, 93898176669],
    [9489417861634552678, 15158637878035490831, 157953668130],
    [1113198223322322089, 17789243229146401893, 34821751405],
    [11190777557146597869, 14677686051252896484, 109964356807],
    [17764553645932638286, 3531237481269211986, 199795678955],
    [14900777150991234852, 8074435404989280910, 235191428767],
    [17364899863357893610, 7086549341467684427, 159437716020],
];
