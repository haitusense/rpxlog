## correspondence table for header

| txt header    | MIR                 | SDR           | WIR |
| :--           | :--                 | :--           | :-- |    
| Cust_LotId    | ?                   |               | |
| Customer_Id   | ?                   |               | |
| Device        | ?                   |               | |
|               | part_typ            |               | |
|               | exec_typ            |               | |
|               | exec_ver            |               | |
| ProberCard_Id |                     | card_id       | |
| Test_Program  | job_nam / job_rev   |               | |
| Tester_Id     | node_nam / tstr_typ |               | |
| Flow_ID       | flow_id             |               | |
| HSTS_LOT      | lot_id              |               | |
|               | sblot_id            |               | |
| Operator_Id   | oper_nam            |               | |
|               |                     | hand_typ      | |
| Prober_Id     |                     | hand_id       | |
| temperature   | tst_temp            |               | |
| Process       |                     |               | |
|               |                     | load_typ      | |
| LoadBoard_Id  |                     | load_id       | |
| Handle_Id     |                     | (not hand_id) | |
| Wafer_Id      |                     |               | wafer_id |
| Wafer_No      | ?                   |               | |
| start_time    | setup_t / start_t   |               | start_t |
| end_time      | -                   |               | |
|               | stat_num            |               | |
|               | burn_tim            |               | |

WCR なぜ二個ある？
PIR = 319

DTRは区別ができるように書き方を工夫の必要ある
DTR { text_dat: "00   0   PASS  Temp_AF  Reg[252-253-254]\t\t0\t\t0\t\t0" }

1. DTRじゃなくてPTRで保存しろ
1. DTRの場合
  text_dat: "[symbol] [key] [result] [unit]"
  symbol : ["result", "time", "note"]

  text_dat: "result Temp_AF.Reg[252-253-256] 0 count"
  text_dat: "time PixelTime 200 msec"
  text_dat: "note comment/err message"

VLB_Value = OK,2.6520,2172.0,g0,P01/L2,[L1],100.0,pt0,[L2],3000.0,pt4
↓
note  VLB_Value = OK,2.6520,2172.0,g0,P01/L2,[L1],100.0,pt0,[L2],3000.0,pt4
light VLB_ParaA 2.6520
light VLB_ParaB 2172.0

1. json

{ "cap_time" : 200, "VBL" : "OK,2.6520,2172.0,g0,P01/L2,[L1],100.0,pt0,[L2],3000.0,pt4" }
{ "VBL_str" : "OK,2.6520,2172.0,g0,P01/L2,[L1],100.0,pt0,[L2],3000.0,pt4", "VBL_lux" : 2.652 }