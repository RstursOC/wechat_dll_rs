// 微信昵称u32
pub const WXNAME_ADDR_OFFSET: u32 = 0x2535848;
// 微信账号
pub const WXACCOUNT_ADDR_OFFSET: u32 = 0x25357A0;
// 微信账号2
pub const WXACCOUNT_ADDR2_OFFSET: u32 = 0x2535B70;
// 微信ID
pub const WXID_ADDR_OFFSET: u32 = 0x25357B8;
// 微信ID类型
pub const WXID_TYPE_ADDR_OFFSET: u32 = 0x25357B8 + 0x14;
// 获取长度偏移
pub const USER_INFO_LEN_OFFSET: u32 = 0x10;

/**
 * 重新查找个人信息偏移地址
 */
pub const NEW_WXID: u32 = 0x025715CC;

// 消息接收函数地址偏移
pub const MSG_RECEIVE_OFFSET: u32 = 0x1B512AF;
// 调用位置偏移
pub const MSG_RECEIVE_USE_OFFSET: u32 = 0x58BFDB;
// edx变量位置偏移
pub const MSG_EDX_VAR_OFFSET: u32 = 0x58BFD9;
// 函数入口
pub const MSG_FUN_ENDPOINT_OFFSET: u32 = 0x58BF6B;

// 消息：类型
pub const MSG_TYPE_OFFSET: u32 = 0x38;
// 消息：文本
pub const MSG_TYPE_TEXT: u32 = 0x01;
// 消息：表情
pub const MSG_TYPE_EMOJI: u32 = 0x2F;
// 消息：有好友请求
pub const MSG_TYPE_FRIEND_REQUEST: u32 = 0x25;
// 消息：成功添加好友
pub const MSG_TYPE_FRIEND_REQUEST_SUCCESS: u32 = 0x2710;

// 消息：发送人WXID长度
pub const MSG_SENDER_ID_LEN_OFFSET: u32 = 0x4C;
// 消息：发送人的WXID偏移，UNICODE格式
pub const MSG_SENDER_ID_OFFSET: u32 = 0x48;
// 消息：内容长度
// 添加好友成功消息：你已添加了XXXXXX，现在可以开始聊天了
pub const MSG_CONTENT_LEN_OFFSET: u32 = 0x74;
// 消息：内容，Unicode 格式
pub const MSG_CONTENT_OFFSET: u32 = 0x70;

// 消息：msgsource 长度
pub const MSG_SOURCE_LEN_OFFSET: u32 = 0x1F4;
// 消息：msgsource 内容，Unicode 格式
pub const MSG_SOURCE_OFFSET: u32 = 0x1F0;




// 加好友 call 1 偏移
pub const ADD_FRIEND_CALL1: u32 = 0x8168A0;
// 加好友 call 2 偏移
pub const ADD_FRIEND_CALL2: u32 = 0x338480;
// 参数偏移
pub const ADD_FROEND_PARAM_OFFSET: u32 = 0x2050570;



/*
发送消息CALL
*/
pub const SEND_MSG_CALL: u32 = 0x5CD2E0;
// 清空缓存CALL0
pub const CLEAR_MSG_CACHE_CALL: u32 = 0x74E3B0;

/**
 * 发送图片CALL
 */
pub const SEND_IMAGE_CALL1: u32 = 0x141890;
pub const SEND_IMAGE_CALL2: u32 = 0x8168A0;
pub const SEND_IMAGE_CALL3: u32 = 0x5CCDD0;
pub const SEND_IMAGE_CALL4: u32 = 0x74E3B0;
pub const SEND_IMAGE_EXT_CALL1: u32 = 0x817580;
pub const SEND_IMAGE_EXT_CALL2: u32 = 0x238F40;
pub const SEND_IMAGE_EXT_CALL3: u32 = 0x131EB0;

/*
修改备注CALL
*/
pub const EDIT_REMAKER_CALL: u32 = 0x4F6950;


/**
 * 根据wxid获取用户信息call
 */
pub const GET_INFO_BY_WXID_CALL1: u32 = 0x13EF10;
pub const GET_INFO_BY_WXID_CALL2: u32 = 0x134140;
pub const GET_INFO_BY_WXID_CALL3: u32 = 0x4FD560;
/**
 * 获取的个人信息偏移
 */
pub const INFO_WXID_OFFSET: u32 = 0x14; // wxid
pub const INFO_WXID_LENGTH_OFFSET: u32 = 0x18;
pub const INFO_ACCOUNT_OFFSET: u32 = 0x28; // 账号
pub const INFO_ACCOUNT_LENGTH_OFFSET: u32 = 0x2C;
pub const INFO_V3_OFFSET: u32 = 0x3C; // v3字符串
pub const INFO_V3_LENGTH_OFFSET: u32 = 0x40;
pub const INFO_REMAKER_OFFSET: u32 = 0x5C; // 备注
pub const INFO_REMAKER_LENGTH_OFFSET: u32 = 0x60;
pub const INFO_NICKNAME_OFFSET: u32 = 0x70; // 昵称
pub const INFO_NICKNAME_LENGTH_OFFSET: u32 = 0x74;
pub const INFO_AVATAR_OFFSET: u32 = 0x100; // 头像URL地址
pub const INFO_AVATAR_LENGTH_OFFSET: u32 = 0x104;
