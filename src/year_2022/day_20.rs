use crate::solver::Solver;

#[derive(Default, Debug)]
struct EncryptedFile {
    content: Vec<i64>,
}

impl EncryptedFile {
    fn decrypt(&self, mix: u8, decryption_key: i64) -> i64 {
        let mut output = self
            .content
            .iter()
            .enumerate()
            .map(|(i, n)| (*n as i64 * decryption_key, i as i64))
            .collect::<Vec<(i64, i64)>>();

        let len = output.len() as i64;

        for _ in 0..mix {
            let mut cur = 0;
            while cur < len {
                for idx in 0..len as usize {
                    let (n, i) = output[idx];
                    if cur != i {
                        continue;
                    } else {
                        let mut new_idx = (idx as i64 + n) % (len - 1);
                        if new_idx < 0 || new_idx == 0 && n < 0 {
                            new_idx += len - 1;
                        }

                        output.remove(idx);
                        output.insert(new_idx as usize, (n, i));
                        cur += 1;
                    }
                }
            }
        }

        let zero = output.iter().position(|c| c.0 == 0).unwrap();

        let len = output.len();
        let a = output.get((zero + 1000) % len).unwrap();
        let b = output.get((zero + 2000) % len).unwrap();
        let c = output.get((zero + 3000) % len).unwrap();

        a.0 + b.0 + c.0
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    input: EncryptedFile,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.input = EncryptedFile {
            content: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }

    fn solve_part1(&self) -> String {
        self.input.decrypt(1, 1).to_string()
    }

    fn solve_part2(&self) -> String {
        self.input.decrypt(10, 811589153).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_input<'a>() -> &'a str {
        "\
1
2
-3
3
-2
0
4"
    }

    #[test]
    fn test_parse_example() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "3");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "1623178306");
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "8764");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "535648840980");
    }
}

const INPUT: &str = "\
5384
-3781
-233
-2155
-1786
178
-2922
-5061
-2874
-3036
-2567
3393
-2801
3181
-4333
-334
-877
-6974
1691
-5086
78
-3405
8075
-1219
7403
6933
-7991
9094
-4302
-1476
4229
-1954
-1700
7328
-4790
5452
-6162
-9718
6295
-5944
5346
-95
-8935
976
-570
5922
-7629
2820
-7465
2207
6369
-8967
-7070
-2807
-933
-1952
664
-2613
-8036
-8840
-2993
-1578
7524
-2528
3503
9432
1323
6410
-6765
-6535
4299
-8254
9908
-9434
-6318
5337
4393
-1765
5143
7966
-8447
-4589
4205
-9517
7867
9177
-7357
6180
-7696
-8424
-4053
2689
-4250
-1284
-6572
9983
-1602
-1175
-6293
4983
-2355
5746
2062
-587
7034
-45
4487
-8706
-6075
2491
-5091
-3201
2042
5952
-3490
-1424
-8234
4340
6660
1627
-8853
7461
-4925
6711
2099
5224
-99
-7667
-601
-9689
-8537
3139
-3685
-707
9590
-5248
-3491
312
5803
-3870
-8288
2157
-8745
-7517
-3925
4144
-5944
4395
-9156
287
-5994
-5328
3946
1930
-9042
8334
-2738
1096
-3001
6401
-4072
-1590
-2228
-1193
5677
-3341
-5291
-6163
6689
153
7381
-8624
9399
-2095
-138
6295
-2525
-2470
-733
9081
2792
8488
-2191
-3619
8639
3852
-4509
-7999
2807
-6174
-8458
-1126
-8083
-6810
6220
-2934
7275
-2752
1803
4894
5975
-3195
7956
-3746
2954
4850
2057
2439
-4046
8373
-5353
-1060
1246
7929
7630
1937
5736
9987
-2123
-6291
-3221
-2429
5090
-6894
-9702
3381
-4575
-8417
6577
-4241
3597
9384
-6169
-9793
-4689
-44
-4690
9637
-6562
7341
3270
6749
1038
-9864
-4453
-1150
-1492
-6093
-9448
-943
-5086
-3487
6948
7149
7527
-8194
3470
4136
6396
-615
2704
6288
9424
7639
7053
-9401
5581
2373
-7244
-7405
3888
-7848
-877
932
934
4417
2922
-6912
-1693
-9838
586
-275
-2887
621
-7669
3105
-6826
-7810
6241
3000
4565
9405
-5393
9949
5922
-3
-9328
3433
-9042
1067
-402
8699
8786
-6568
-6654
-9658
-6292
6136
-6402
-1602
5016
1980
-4774
1841
-6002
6039
-5675
2798
3154
4062
5133
-674
-4355
-2284
9944
-5675
-2285
373
2337
-2569
2982
6443
5568
-5056
-6894
-6112
10
-5279
-9832
842
8971
-4937
7687
-1527
6410
-9708
2581
4494
1080
-44
3135
7045
-2542
-3682
-6246
6528
6652
-8312
5379
-5553
-897
-4700
-6619
-175
-4571
5191
3655
-3243
5299
-7361
6089
-7500
-7989
4429
-5022
-2349
-4812
3979
-1499
7717
-7674
7401
-5959
-7044
7613
-1685
-7889
4453
-8896
-9002
5890
-3778
4040
4833
4880
81
8786
-3167
-1717
-7784
-8072
-4675
3828
3486
-4230
-1140
6385
9902
2549
5164
-8308
9019
8714
8007
-410
-5181
-1671
-3781
-831
-9817
-531
-8218
5171
2249
5388
7537
4263
-444
-4763
1579
-8967
-8338
1662
-3969
8852
1792
3010
-7005
2432
5395
-5196
-1008
-7517
6011
-94
-3197
2598
909
-5196
-7070
322
5319
-7130
6376
7095
7264
903
-9684
-7158
-8677
-6856
-9771
-2984
-2925
8275
5630
-2526
-4373
2114
-5303
-377
-6213
-1478
3886
-2281
-7727
1208
-159
-6576
8822
9019
-2070
9871
8916
6752
-8627
4720
-9325
-342
9431
-1685
7661
2352
971
7598
4438
-3554
-7833
2629
-7208
9185
-6388
5572
-8900
6229
6729
3083
-5396
-1091
-5909
-7306
-6143
-9408
3107
8320
-4572
1667
546
-4490
-1700
-3048
-8288
-5210
7923
-1167
-4237
7267
6070
-2478
-4851
-7569
-26
-8857
4789
8287
504
6591
-3746
9802
-686
-2324
-4325
-6429
2890
-974
3948
-143
-6893
1318
9844
-9042
5856
8259
-6361
6508
-5587
765
-2777
6668
-2687
-1057
-7533
2598
7588
3470
-502
-5196
-75
9417
-3896
1436
9916
6621
-7629
498
7451
2283
-6367
-3436
-1914
6240
1015
-4890
-2808
993
5398
8812
-6769
5181
9458
6263
3139
9748
-8413
1038
-4654
-8195
-6970
3970
1370
-1494
1209
-4571
2050
-138
-7301
-2227
2410
1717
5185
-3051
7537
9263
-828
-8424
-7408
128
2014
-3402
-704
4895
9354
-8418
3520
9557
-4174
7945
1388
3798
3824
-641
-5089
-1765
4906
-5134
-6263
6363
-8896
-5909
-5247
-9616
5614
9735
9585
4952
-8317
-7744
-5002
-5900
9845
3131
-9718
-4379
-1792
8137
-1953
-1963
-2594
3610
-5360
9889
-9309
-9684
-3898
2842
-822
4593
8860
-1936
8007
206
2439
1582
-1520
9655
-4283
6673
-2061
955
5331
3086
-4590
6122
1548
-5951
4248
2449
9260
-9560
-7130
-7068
5828
7653
4522
-9964
-9123
-6675
8711
5828
6357
-334
2612
-405
-1334
3141
2739
-5740
-2089
1806
-1609
1608
-8433
7618
-1671
1568
-5041
-4855
-5643
-163
-1207
-8294
118
-1931
7262
-3201
-1419
-7081
-3195
3111
9517
8172
633
652
6492
4809
119
7319
9447
-9995
-8534
-9
-5860
1476
-1833
-4703
342
9563
1197
3466
-299
4275
-5269
3559
-5910
-3476
6712
3590
1842
-6246
-9025
3639
-7677
6836
-4807
-2384
-6937
-2161
5898
-615
8191
-2897
440
-2754
-1896
-2868
8141
2918
875
-4005
-1516
-4879
-1549
-4794
-1600
373
-2551
1220
8474
1129
-4227
-7896
-4106
1122
2804
-2623
-9625
2937
1490
-7550
9924
-932
9682
-426
3878
-8605
-3944
-2750
5306
2577
8135
7546
-2905
-5601
-8340
-1423
-8294
8984
-765
-1850
-44
1595
2101
-3674
-2745
4115
-921
7021
-7348
-6384
7018
-6671
320
2797
2264
9731
-687
-2566
6060
-6427
-4199
-4150
7850
-1004
0
7687
-5148
-4814
-661
2018
-7567
382
-7420
-3884
-2801
-5337
165
7737
9052
7565
3670
5712
-9623
9246
7038
1145
-4446
3799
-3405
4908
-6528
-1503
-1986
-8431
8401
-5137
6098
3785
2549
1651
-465
7221
-3111
4395
5145
5641
5840
8588
-3203
3139
6712
5400
-7554
5238
2729
4020
-55
-6866
-2993
-4983
7139
-6009
-5959
1170
966
3586
-7401
5526
-8944
-9660
-1604
-2650
-7661
5490
-1274
4878
-4385
3939
-6317
8783
4062
424
-9567
472
7535
9733
-8703
-2997
7769
6709
-2375
-4814
-8672
-9984
7042
5071
9818
-9202
4604
-4492
-7280
-6973
8780
-6337
-4488
-5908
1875
3878
-8931
-4694
-7352
-1934
-2561
4897
2840
7293
-1960
7432
5692
-7430
1579
-2204
-363
6633
7662
3960
-6433
-7295
-2445
7853
-9712
5899
-5423
-8457
-669
-5414
2198
-6390
1674
-2479
1128
-4454
-6152
-8081
4395
-6595
-6713
-8298
6425
4487
4541
6515
-2102
-1694
-4702
-9922
-3138
5331
9072
-9671
92
-2161
-8631
-3933
-6748
-4386
-7420
2309
-5470
-9578
-298
-1996
-7119
6523
-3193
-3898
5144
-781
-2460
7346
-8340
69
-4700
-3459
-1896
-9705
-6144
5216
-1334
-4425
8875
-2907
-5544
-9990
358
-554
-9585
-1634
-4272
1674
-6145
-1875
8419
-2425
2592
477
7224
5630
-8382
-9834
-3345
-4442
3818
996
8104
-6426
-933
-1566
6449
9081
8114
2446
-5061
5010
8712
9463
7143
9153
5830
8022
7227
-2380
-5968
-5544
-7081
-8548
3272
-4599
6147
2446
3683
1159
1252
-5266
3852
7456
-3301
2171
-6457
-5021
6836
6831
749
-3235
-4768
-1252
-1398
6414
8777
6722
-456
1745
9098
9672
-4328
-4696
-8486
-3674
-8631
-6394
3034
7207
7829
3509
-7572
-8160
-2557
2927
-9
-7779
-2425
-7529
2247
-1049
6289
-4188
2612
-7979
165
5504
-5362
-8146
3655
-4296
-6627
-2754
8562
-1035
-6439
-9409
-7151
6300
9342
-8619
2294
7689
-8288
-8913
7205
-9362
7196
546
-4694
-7198
-2130
5277
-1389
8918
-5171
-2392
-6334
5544
2805
-952
-6519
-3853
-869
3351
-2192
-3449
911
-1022
4839
-1453
3204
-1609
-4713
-1755
9992
-3313
-8112
-743
-1748
-4386
-1481
2676
932
-8418
4356
1923
-6332
-532
-4689
-4035
-1013
8439
1621
694
-8205
-152
9348
5418
-1609
1721
5373
4482
-7376
3717
-7517
-6477
9503
5499
-987
4161
-6278
2981
3682
2520
-6121
9532
-5467
7621
-8073
-6000
8073
-3652
7420
6616
-5841
-4835
-9542
-9808
1318
-5617
-2897
-9886
-5232
8121
4141
-2388
-5719
1368
-1341
-4346
9797
-7647
-3414
1526
2695
8402
3533
-6929
8800
3749
3421
-8735
7058
-7751
-3478
5480
-2603
-8451
4189
2505
-5552
-7751
-7103
-8635
-179
1595
-3414
-8794
2982
9326
4901
-4097
3299
-823
5035
-2900
-549
-4107
-6534
-6233
-8164
3961
-7255
-3380
795
3447
-9695
-397
5581
-9783
8539
-1412
-6794
478
-4918
-2710
9737
4810
628
4269
27
-9370
6327
4082
7113
-9901
914
-8953
-5539
-3587
4403
-6886
9785
-7288
2101
2266
5843
976
-2255
-172
-4686
-8110
-1927
8161
-92
-1378
8069
5362
-1126
802
181
-8389
4149
-1398
-8327
5995
3154
6530
-1732
8982
5407
-9470
8783
5086
-2183
4537
-9056
442
2670
-4710
1915
8486
3397
-2302
2841
-8344
-8704
-2224
9101
1420
7341
4875
8628
2607
1251
7875
8759
-6223
2792
-132
7252
125
7604
-5123
1865
-4458
-7211
-5151
5083
2643
-2605
-7060
7051
-8829
-8046
-1741
-6299
-9274
-44
-7619
-317
4318
2030
-3058
7636
-3539
-5295
1187
5606
-340
4653
-2020
3397
6359
7190
-8306
422
-7301
-9706
-5089
-5014
-99
5959
-1609
-6285
7519
-6114
-7573
2457
-397
4371
-6376
2748
-8693
-6898
-7595
-1167
5160
5581
-5277
8961
-5565
2087
-5072
4033
-1598
-9042
-3521
8754
4456
-4023
8711
-402
-2408
-1983
-8930
1303
-7307
5876
-3024
-3256
2383
-3753
-3676
-1995
7058
5777
-7805
-9475
9695
-8905
-7873
5463
-9467
8574
7418
-3811
6326
-8610
-932
6738
7557
9603
3091
-7046
-2551
-3385
3777
3407
6589
-687
1437
9859
-9060
5564
-7888
5856
-2167
-8509
-5284
-175
-2476
-1794
5746
-5243
-8525
8518
-9504
623
5197
-474
-2479
-6451
-9623
1694
-3684
-8652
-7053
-7966
-5579
-7789
-5617
8994
2139
2466
-5198
8071
-366
-5635
842
8466
-8594
-2061
1522
1041
-6033
-4941
5609
-535
7058
6633
-1248
5869
-7960
2510
741
5717
-789
-414
-5072
3319
-3001
-4933
-4890
-3382
8008
-5544
7010
-876
941
6194
1595
321
-7979
-7216
9101
3015
-9691
-7684
-4073
-9182
-7475
-8356
-1870
2176
-9571
811
9406
-6162
-4492
3761
-4525
-4302
-9402
137
4805
4927
6357
-2865
9504
-9193
7293
-8631
7196
8180
-2714
3471
8589
-7979
78
-5317
-8288
2760
-8745
-6678
-3902
7201
9
5974
-3155
8512
8128
-8031
-152
5461
-3459
9655
-8631
-910
-7481
-7911
-5933
-9578
-7647
6463
-2092
5802
-776
8653
-7677
-6992
3769
8474
-6779
-1476
-5135
1208
246
-7617
-5347
-1830
-5425
4577
-9993
-1296
-5360
6016
-9984
794
-4268
-340
610
-7349
-6211
6448
-5961
-6933
-1453
9342
-1817
-2934
-4402
-2257
-5964
-4718
-1008
8529
-5396
121
9806
4799
-1609
-698
1347
-6267
2201
6026
2907
-9389
-4756
948
-5119
7354
8287
4242
4313
-4043
2712
9013
3981
-9964
7613
6727
6082
5907
4270
-6429
2661
8179
6669
288
4398
9924
-9428
1905
-2714
-7548
-8164
-8817
206
-5021
-335
-7717
-3269
9604
2827
-5491
-3191
-8191
-2312
-932
-9817
6016
-5296
-6780
3104
-1983
-5842
-5567
-378
-4129
6127
3893
4697
-5617
3825
936
1251
2546
-9559
9284
-6780
-1478
-2857
9820
-3283
-6216
-4998
-9343
5050
-1540
169
1495
5952
5004
6642
-615
2406
-5150
-1383
1897
267
6183
721
-9538
3469
-9309
-5148
2589
-8480
-1767
-6826
-9825
3590
-5198
9859
-595
9930
-5823
-9947
2999
-3723
3063
7958
-1870
9098
8556
5842
5594
-8073
2712
6530
7491
-6794
6231
2785
-6401
-2962
-2802
3852
3181
-4952
-6612
-7162
5856
-4043
9981
-8400
-7833
6531
-5912
436
474
-3721
-8184
9992
4536
2922
826
-3195
1840
-1983
-3301
5836
-9723
-7133
6240
-3520
865
5499
3914
4447
2205
5760
7621
1991
-1384
-6967
6355
-9523
-7072
-4833
-9532
-466
4684
7771
-7414
8487
-3235
-4920
7618
-9359
9864
7544
-1106
-8997
-2437
-6588
9490
-197
401
-8950
2997
6722
648
-9025
-3172
-3313
-6293
2700
842
-5410
-7819
8428
-6818
-1398
1246
2042
5317
8893
9060
-9434
-7052
-4554
6028
-510
9593
-2327
-2752
4871
8014
5816
-8509
1229
5922
-352
-8170
6271
-1341
1844
-3137
-9579
-8734
-5802
-2062
5614
-378
4983
-8684
8789
6872
-6028
730
-8803
-8933
-5366
-6717
-3781
-5014
-9031
-5847
-8776
8466
1329
8320
-2183
7420
9748
6978
-6644
1031
75
4357
-7287
-1817
7205
-995
2568
3933
4787
-2802
-3402
1934
-2024
4297
5407
3250
-6300
-3135
-2271
-1035
-8694
-4129
-426
3996
3105
2556
7337
4823
-5903
5844
9223
-6762
-8694
1923
5547
9087
-7211
8900
4408
-5512
-7602
-9481
-5806
-1150
-507
374
-4833
7598
1933
-7692
8014
-6978
4536
7852
8940
8179
5082
-4501
2216
6016
-7240
1256
1647
-2714
-9784
-5818
-9504
-9244
-9834
8766
-9017
-1890
1281
3828
-4362
-7215
-4755
-8184
-5407
-2384
14
9225
-2241
-8933
-2564
8269
-8176
-3722
-7546
-9991
5171
207
-7928
1106
-2849
-9538
-2359
-3407
4525
4654
7604
-1084
-661
5687
-6644
4074
2720
-2533
5140
5206
-1781
-8017
-210
3521
9621
889
-8539
2997
-6497
-7020
-6528
8279
4910
-4700
-6839
9671
35
783
-3974
-7691
-1566
-509
-6580
-6592
9769
-3963
-6450
4335
1168
2712
-3066
9637
1934
3000
-750
-943
-8194
9229
-896
-181
3890
4608
-6825
-1442
2375
8461
2563
3539
-6074
-4886
-6024
-1693
-8509
2160
-8283
-3237
2581
-4691
966
-8393
-3997
5171
5677
-2724
4831
9825
-1861
-457
-4333
-5009
-822
4192
7021
-3997
-3314
-9964
-4318
7619
-8575
-6625
9320
-9956
-5637
-5140
2282
-773
-1463
-4937
-6535
-8176
8997
1174
2881
-4319
-8844
4430
7811
-2815
8337
1316
-8825
-4150
-285
-1692
-1794
8191
473
-8973
8703
-7490
-932
1069
3476
5895
-9548
3398
8987
-7551
-9070
-4027
-5745
-7539
7221
-6131
-7822
-9171
-4680
-2592
-4948
-7274
874
-8892
7870
-7291
1973
-907
5800
-1161
-4586
-5620
7499
-822
2643
-2529
-305
-5626
-6462
2682
1988
-6202
1201
9422
2237
9083
-138
4096
8167
1905
9774
-2801
-7199
-3211
-2576
-8503
9001
4297
8562
-8253
100
4685
-8786
502
-7717
-9366
1149
2030
3005
7105
5425
-4824
4471
9234
-8862
-5648
-6403
132
-8610
-5423
-8863
988
4649
-8435
7349
5257
-8874
4978
5932
-5818
-9147
4961
-152
-9307
-4459
-4926
-9964
8825
6187
-1594
5011
-2834
-1175
-4005
-2623
-7774
9966
-2941
-2257
5995
-4350
5641
-5635
-3704
-7980
-8695
-7007
7921
9593
7557
7945
-4960
5377
658
-9281
-7158
7310
-6221
3645
-1314
2247
-377
2224
-9832
5712
7585
-8909
-225
-993
7016
-3039
-3478
1313
9699
-973
-6194
6905
-7003
-6626
581
5922
5273
6229
3319
-3348
-4032
-7679
6841
8612
595
5883
-4420
4575
-3969
-9321
-7472
7729
-5696
-6398
-4010
-4566
635
313
6052
8977
-7783
-230
1865
1304
8931
5760
6861
-4920
-8684
1288
-5766
2643
-1893
-1635
7704
-7444
9160
-9331
2342
-7717
9265
-5253
783
9260
478
3098
2173
-6619
-9137
6931
-5170
-8396
-5198
-3296
-7803
-1424
8104
-4397
4531
-6039
-5637
-3927
-5546
3818
-9130
4408
-233
-7075
4172
9533
9851
9832
1579
7405
3139
-8297
8277
-4032
8103
-8706
5642
5346
-934
-2914
4335
-9833
2168
-3888
-246
4034
-1853
-9147
9909
-9397
-9523
3898
-9422
-6306
-9901
-9486
2603
6034
-3310
-1748
4269
1685
-5295
8621
-9930
-4092
-5016
3825
-7307
4117
-4992
-9922
100
-561
2618
-6574
3811
6836
-5473
-6958
-5891
1614
-3997
4663
8662
8441
5845
-411
-4729
-4007
6361
185
9597
-4325
-3237
-305
-1314
-8031
9383
-4319
-1265
3885
889
7513
8613
5134
9228
6693
6515
2872
-4772
73
-7120
-5094
-6663
1742
-6232
-7022
218
-9996
-9784
5674
-876
8797
-7258
6445
-3448
-3539
4981
7152
1916
8750
8899
2058
-3730
2862
9829
1923
266
-5393
7967
-611
-5903
-1346
-5255
-1216
-3880
-68
-9692
-7546
2933
9496
9076
-1982
9655
-2183
5843
2603
3421
6194
-8701
-5555
6669
-5697
-6560
-8868
6229
-6361
-8695
3312
3898
6633
-5126
387
7324
3726
5346
-2715
-7246
8181
-5498
-7007
7473
2132
7586
225
-3266
5086
1865
-9147
2663
4259
387
6488
-9886
2982
-4033
-9328
158
-1330
-4328
-1004
-6577
-363
2583
-796
-2424
-334
-8743
3950
-2920
9654
-4188
4906
-3209
-5696
-3374
-8269
-4322
914
4785
-4302
-4983
5712
2015
-9567
3314
-6457
-3962
1220
8054
-3797
7596
-9589
1246
3238
7945
-4266
-2314
7358
4565
-1435
6846
5786
1674
2997
-2771
93
-7420
-1188
-1035
-5194
2588
-2028
-6215
9047
-7961
-8117
9348
-7137
-3402
-9025
-5168
6136
-2853
-3664
546
-3180
7730
-1073
4115
-2114
-5289
-8456
-3704
7082
6873
-7612
-3855
2325
1696
5331
-1927
-6242
-5140
-5203
-7211
4387
-7081
-209
4826
500
-4407
-8786
7585
-8991
7354
-4685
-1229
5356
-2062
7428
-1758
-5126
-4124
-465
4670
9348
-2828
-4833
-7651
-8486
2216
-8754
8995
-3856
-6025
6889
2883
-1532
2907
-4187
-5618
1339
-1072
-4061
4708
5474
581
-3997
3719
3043
-909
8227
2914
7586
-7240
3193
-5148
-4814
503
-6907
3707
-2792
-1346
-1437
-7721
-6970
6710
-80
8800
-1954
-4835
6030
6933
-1167
-6024
783
-6301
-995
-5544
9307
-1941
3759
-3653
1727
2087
-9470
5232
-3626
6531
-3342
4823
-445
-3060
1422
-9322
-3060
-6974
-8344
-1728
-9610
4616
1033
-6033
3010
-7640
5065
-2302
3098
-5407
-5256
9072
-7551
-1829
5849
-1281
9095
-99
-3626
4604
-220
-6561
-8376
-7205
3351
3777
-2989
-8745
-3648
-6634
2022
-1419
-4182
-9319
5035
5849
-9625
-6613
-5408
-2720
5821
-690
-9553
3015
7964
-3527
3501
-2788
8056
-8945
7957
9182
-8047
4863
6229
-5148
-2460
-4586
-3234
3562
6567
3935
3766
7852
7011
-6216
7776
-8047
-3070
-5508
9095
5180
-4168
2621
4843
-5199
-9181
8530
-9784
-6919
-1006
-5608
2623
-6540
1015
-688
-7801
-8287
-1644
-4858
-1065
7612
3270
3461
-3885
6371
-7201
5302
5544
1825
-8973
4578
-82
-1094
-5322
858
2222
-6886
-8555
1460
-8112
-9589
8266
3412
7941
-1834
-910
-100
-7786
-7498
-8703
-9548
-1954
-1273
4301
8574
9332
-8203
-3342
-4174
2057
-4790
7272
-9501
5086
-1033
4778
9266
-4233
-6375
7921
-1990
-6425
7869
-1889
-9519
440
-3744
-3145
573
110
-8841
180
5362
9159
-6233
-4755
-6648
-6420
8193
-9879
4494
5198
-6024
6896
3276
-6678
492
-7442
-571
4720
-6860
8393
6933
-2269
2181
5150
6229
-3432
-6975
-4182
1460
8852
7555
7640
6499
-3705
9743
6425
-2151
471
-2181
-7310
-5061
4325
6957
-6551
730
-3046
5836
3047
-1653
-9337
-1376
-9019
-4589
-5718
-7981
-407
5993
8147
4635
5993
2798
9680
-6246
-4956
1976
594
774
-2155
8662
6749
-3398
-470
2012
4951
7258
4881
5610
-8022
9712
-3335
125
-115
-8631
4578
-5818
3950
-9538
9237
-5188
-603
-4829
-1461
5045
5082
-4509
-7579
8227
-4196
-962
-450
5423
-6728
-9389
4340
-7361
9733
-4998
-6167
1119
-6972
8977
-3611
-1012
-6109
6344
874
4511
6779
-8115
-8391
-6074
-538
2381
8034
-5118
8914
1615
-4617
-175
8055
2573
7461
-1759
2592
1745
9748
-9684
5319
-3952
8693
-6300
-9062
-1906
-4509
-5787
3791
1528
-5247
-3623
-7786
9340
3896
-1314
941
8612
3447
1532
9990
2589
1667
1891
-3651
4242
-2303
1919
-9327
5586
8745
4494
-1578
-8900
-1364
-2777
2410
-9751
-8189
-3616
472
-3145
-9408
-618
6087
8800
-5526
8493
6087
2339
-1261
-1435
6779
-2920
-2855
-4553
7305
7396
2481
-8930
461
6937
-7428
8641
9142
991
4491
1734
-4033
-977
5704
1544
-2123
-1764
-2299
6957
4758
-5046
2851
-1971
-9370
-1171
6357
-8848
-2590
7625
-6459
-4257
-5134
-6929
-5021
9972
-4887
-146
2869
8734
-7426
9682
9152
546
3886
6559
3363
4641
9202
9787
4663
9383
-5083
4473
3471
-5672
8815
8686
8380
-2249
9511
1082
9631
-4767
-2365
-6079
9606
6892
6835
-77
-71
4078
-7103
2252
7211
1593
-2740
-7979
8021
5601
-4603
7765
-9096
-532
-698
-9366
5614
-5315
-4281
-2060
-6705
-6601
3104
3274
8487
5786
-6794
-397
7054
1037
1306
-8276
-6074
-1224
8592
9371
-5516
-2290
1176
2712
8731
9350
-2992
5915
-6534
5378
-4955
-3676
492
1644
-8914
2966
6727
3716
955
-6769
9935
-2424
6492
-4307
722
-7885
-9702
-467
4615
4776
2137
-8036
5013
-4603
7726
8716
-2916
428
8004
-5417
9491
7726
-7465
-3303
-1057
7030
-9718
2353
-9203
-8388
3351
2156
-2312
-3167
-4472
-6450
-4202
-2354
8613
-7618
-6912
7946
9319
-4082
-2623
-6024
-6968
-6180
4470
-9258
9332
5158
3960
706
7381
6531
-5484
1884
8722
9920
4491
-4031
6170
-1463
-9489
-6648
4826
-8552
6890
2820
9768
-2563
9142
-597
-8859
9447
1685
7931
-1091
4138
-1672
-5781
6910
3173
6890
218
7298
-2801
-4205
-7349
-9397
9249
9631
2515
-406
5370
-6622
-179
3461
5768
2748
1652
-4230
5338
5378
-2873
-692
-2801
-9025
-6749
-4092
3597
-1950
-2932
1905
2751
-9503
8137
-6422
8777
-4529
-3061
1626
-910
-5539
-3055
9072
3539
8593
7396
5926
-1333
-2460
5198
-2636
7850
-9549
9999
4565
7576
8334
-4772
5877
-6009
682
5715
-7683
-1769
3873
-1635
-1118
-989
-8197
7119
-8176
1263
-3076
5124
7248
-3383
-8383
7625
-8299
-2210
-1855
5086
7674
3092
-5190
1944
-1608
-1583
2695
-8731
3007
5803
-9342
964
-2277
-3221
-7068
-6826
5840
-3122
2949
6896
-4566
2933
4515
6370
-3653
-8963
7095
-6033
3667
-3587
-1432
-2192
2069
-7787
-9274
2432
-8205
7847
2541
8947
-6692
-5675
-8443
4526
6589
9177
4173
9990
-3865
4420
-5086
-4120
6876
-8829
-2710
-4420
3785
150
8589
5711
4079
2751
2076
9107
8249
5008
-8542
2890
-9056
5490
-2647
1224
3358
5502
9587
-5810
6289
9865
-2620
3772
4641
-2807
5218
4200
9952
-5185
5086
-6918
3450
-9112
-9817
-3300
-3407
6704
-4907
139
6192
-3724
-2177
-6626
-9075
-3746
9828
-8658
-8112
-9385
6534
5946
9101
9843
-2486
877
-8675
-9112
-9068
-2146
2813
4344
-4188
2137
-8667
-615
4453
-6816
-9252
2641
-6878
5786
1875
5895
-2066
-8555
6057
1313
-5547
2670
-4107
5668
5327
9162
5912
-619
6752
-9886
298
7156
-7149
-6333
4339
7749
4132
9733
-5831
-9832
-9580
-9649
-59
6344
-7428
7579
-159
-9486
-2349
-527
-5425
6901
8520
-7405
-391
185
1674
-7386
2869
-2651
-1601
7666
3947
2611
-2175
-9399
3192
1842
9174
-4458
-456
-9834
-1671
-8546
4338
-9428
-9957
8839
-672
-5922
5538
7018
-5407
-543
8257
6495
7006
-6678
2893
-6285
-9527
8390
3769
8759
7514
-6627
-6169
-1364
-9268
-5903
-3195
6268
-9304
-9684
-1160
5115
2224
5793
-7686
876
8861
9377
5283
-1746
-6550
-2360
5400
-6050
-7089
1091
4500
-69
85
-7428
-607
5183
9672
1255
9200
-4815
-9090
-7183
-613
27
-4763
-6333
2808
8402
9330
7398
-7339
9326
5762
-2228
8510
-7824
-8297
6889
3047
-1728
9181
4541
-2096
-6442
-666
8461
7577
-611
3583
-1432
6333
1993
7158
5772
3964
-2638
-4697
-9585
-771
2474
4190
7025
914
-3433
-7003
-9792
-8695
-7521
-9002
-4772
-2224
-190
-795
-4762
-6081
-6648
-7793
-1123
5124
-5617
-1173
-128
4271
351
4357
-6365
-5626
-8418
8955
-653
-4477
-8605
623
-6112
-7258
1674
3847
7749
8227
1176
4422
-9849
9765
2937
9864
4062
4850
-2561
-6548
5125
3698
-682
-6055
-1874
-5899
7695
-5892
8179
8069
-656
-1035
-4106
-8884
3890
-3730
-9834
7695
9655
-8843
-9601
-3704
-737
-9997
559
1593
3539
-1488
-9480
1993
-507
3587
2150
9323
5897
8420
3312
-2110
9593
-6221
-9792
-760
-7130
-5835
-823
-2269
-8764
7431
6357
-6394
6435
1597
-8806
-6716
-9264
7393
1309
-8904
7361
-9112
-8786
9771
645
-4072
-77
-8874
-6039
-3593
-2680
-7753
7546
-9075
298
-8552
8331
-7751
-1548
435
-2780
-6794
-919
1764
-87
-2598
2937
6611
8014
3296
-4369
-5563
7353
6448
-9954
1187
7617
-8146
-9706
4361
3564
4487
1460
7337
3043
-4193
-4060
9238
-2078
-4213
-6923
-9815
6560
-2164
7341
-6717
-498
6939
-3878
-2257
9710
-5951
4259
4983
8222
-3012
-4357
6137
-2934
9180
-3364
9703
-952
-2309
-5662
-8538
3442
-7867
-7887
-2089
5014
2137
7176
-9996
7511
6125
5428
-9832
6834
1107
4536
-1260
-8997
3262
-1446
7042
9820
-2669
6774
-1401
-966
-1641
-1118
-1312
-2657
-5745
5373
-4072
-8073
-7475
-4420
2406
-1650
1958
9387
-4759
-1299
-6074
6103
6531
3756
9317
-9503
-7098
-917
5455
-1035
8883
-4005
3497
-646
-5148
-8313
4979
9288
-698
-7751
8463
2418
-5137
-6898
2862
1884
-1899
-7913
2955
3670
2589
1739
7381
-8817
-9971
-3208
6219
-1461
7618
5396
-8970
1641
3149
-1364
4646
3531
-92
-3030
4131
9434
1562
5704
8035
75
-210
1165
-2362
-7423
765
-7249
1562
-2857
-9918
-4870
-7020
-8026
-1493
-7992
8464
-9087
-3800
-6572
-9706
9587
-5648
8951
-8341
6890
8801
6378
7223
-3880
6763
4592
-5951
-3436
4230
1652
4299
-2365
-8817
-9417
9447
9984
-3587
3898
-4065
-9528
-6967
5046
562
8094
-2937
1685
-876
7852
-7149
-3897
-9717
-4008
-3186
8691
4826
4211
2253
-5623
-3520
-3682
8460
3569
-8195
-6956
-2771
7182
-3541
9961
-7674
-2925
5228
-550
2498
-7692
8422
3726
-2345
573
-1814
-3001
2362
-2733
2669
4677
1537
1560
-6840
-7219
-9147
3777
-2891
1113
-585
9593
-1203
-2197
2904
3393
5245
-3683
-1667
-7477
9076
-1093
2310
506
-128
4525
7782
-5069
941
-661
-770
9893
6813
7337
-2217
6738
-9359
2707
-298
4613
-1622
8643
-1232
-2997
-7934
-813
-5416
-9886
2369
9218
-8157
440
-9056
9340
1204
581
8904
3824
-2325
-6299
5611
6369
2493
-532
1522
2783
-3614
-9792
9284
-7832
2300
9884
8896
3538
9562
3250
-4403
-1703
-4815
-4937
-1889
3869
-8463
8588
-2841
482
259
-1331
-6590
-6874
4919
-6488
7604
-1118
-7420
-3685
-4193
7706
2558
5858
2127
-7291
-6367
5762
-3243
-6220
8350
2820
-8704
-4822
8141
6046
-8534
4972
1674
7351
4688
-3029
-9470
-9864
8193
1314
6759
-9517
-8415
3024
2294
-1422
-599
-1950
-7103
-3445
-9705
-8945
-9394
-4814
7519
-9712
1641
472
297
-8700
7380
-5994
-9385
4040
4040
-4329
-109
2080
-1811
5228
4800
-6367
-6776
7621
-1328
-5385
9804
-6836
-3301
-3784
-2134
9722
-454
-3626
-6210
-7137
-1464
8711
4162
-7692
-5744
-6613
1976
2377
4314
-11
4854
3319
-7444
-2074
-5773
5564
-5750
9277
-1005
-7553
7329
2768
5673
-8449
6708
-9473
8283
-7017
-1717
-142
-5149
4097
738
-4775
1540
3364
-3145
-1710
-587
-42
-2001
-1996
5142
8879
-3921
3497
5712
4192
-8628
9903
-4908
-5126
735
-7565
3131
5551
7456
-7996
-3635
-7500
7749
-7295
2773
568
4034
5057
-5745
2552
-6498
-237
6948
4243
-2705
9076
2106
-8297
5692
9107
-3855
-9461
6562
-2567
105
-7528
-7181
-6967
-8942
1309
-7943
9645
2315
3649
-8800
-637
-5589
628
7564
4020
-3735
-3705
7905
-9721
-7405
5337
-2941
-9203
9555
2998
1295
-3151
2055
-1368
9107
-8631
-2576
-8930
9229
-4582
-3649
6597
5261
-1900
-4148
-6126
4494
-59
6836
-8927
562
5684
2541
-5069
741
-8584
-6118
8423
-596
7867
-5937
9069
-9971
-3140
-8744
3815
2375
-4590
1281
-165
-8191
1637
-7428
-391
-8061
5831
882
4907
3539
-2301
3818
-4488
1067
-3928
1526
1187
2225
-8308
-1781
5502
1306
-3651
5081
6531
-5165
9229
-1934
-2338
-3662
-4472
-7595
5521
-8468
-3863
-3878
8611
3071
9938
4608
-6380
-6319
969
-6272
4259
-7572
-7935
2855
5106
658
5936
-8435
-3683
-190
8606
4765
8193
-4589
137
-4418
4798
-928
3071
5608
8563
9682
2520
-9281
-2585
4833
-8351
7937
-6617
-8575
-2603
-2617
-5046
8292
1424
-6390
2272
-8863
1893
-538
7384
-3237
-6578
-3313
8600
3876
-1494
1168
5524
1694
-601
6105
7604
2278
4240
5163
7945
2704
-5854
5544
6043
-9611
-9997
7956
-4687
-4859
-7472
1783
-8950
-9560
-3626
-6694
1958
8745
-2349
8685
-6762
-5513
-2773
-5308
-8112
842
-5823
-7599
3679
-4321
-1437
1239
-6289
-936
-6527
-1740
4196
2428
2418
1201
-6242
-1346
-2894
4487
2542
-583
1953
-936
-8570
-5881
-4706
451
7951
1268
-4920
-2406
-2635
-5001
-6836
9268
7428
1421
6577
-1549
-5556
-3880
-2788
-8533
9259
8365
288
128
-1682
-5405
3731
-6242
3662
-2114
7624
8461
3580
4003
-190
-1741
1437
-1606
7945
4846
1679
199
7683
-4221
4900
8656
3977
896
-6018
7211
-9964
-1869
-3205
-4288
7965
6638
8328
9832
-585
1544
-9832
445
8799
7661
-2204
8639
-2382
-3887
9204
9150
-3277
-6917
255
-8731
-6398
-7960
-197
-993
-128
1055
-3984
-4676
6387
3737
-6351
-5251
-8677
-483
4269
-7052
3144
5581
9294
7652
2156
-3651
-7756
-7867
-1692
-175
-7137
9655
5952
229
2543
8719
7756
-2018
9338
-9964
-2436
-2928
8004
-4407
-4553
-5822
1679
-7881
1021
-197
5802
1254
5793
7337
3822
-5504
-3345
-9330
-4840
9113
-4832
8075
-4188
5325
-1378
-7244
-5115
158
-4812
-9260
5319
2560
-9918
3105
-5989
8094
-585
-6183
3852
-5908
-2902
-7864
7479
2345
-4380
-297
-8942
-1167
-7568
-138
-8081
829
-4685
-1
-1447
3567
8048
-8860
-1261
6550
-2654
5731
-8065
-2026
-7717
1728
7661
4112
-8493
-7864
-9393
4929
-3009
3837
4255
5775
9643
7868
8902
-4042
2974
3212
-7062
-633
6192
-4665
-4293
-9587
-6912
6435
4843
508
-6380
-5393
-4381
-8536
4154
2353
-7843
-606
8750
3922
223
-4331
-6534
-1177
7533
-9445
5674
6359
-3404
-5767
4850
-9096
6006
-9776
-8374
-5061
5747
-1671
-8944
-5645
9471
-8594
-3165
-4385
-9322
-8900
2581
-9845
4439
-3009
-7798
4131
-4588
5797
-152
3957
-8288
-1364
4074
8325
-8344
-1972
2997
8955
-354
9454
-9705
-5770
911
-5055
-4884
-9534
4230
180
5809
-7896
5505
-8081
4339
-3131
-3277
-762
6147
1321
1421
3930
1065
-3348
-5966
-4303
3967
3135
6424
6715
-5426
8198
-4465
7707
-9792
1015
-2154
8768
3243
-2752
-1155
-7747
-7786
5845
-670
-3616
1123
7964
-3517
8808
-8857
999
-6572
8497
-2505
7683
-5514
-5196
9326
-2598
-7887
2173
-7334
8794
6727
-6044
7633
5271
7030
-1629
5521
-987
7967
7897
-8036
6262
-6212
-8677
-6644
508
4211
9147
7200
-9354
-5303
9567
4173
1732
-2110
-6167
3590
2623
4549
390
2493
451
-7691
5094
1067
9926
-1216
-9458
422
1304
703
-8122
-6688
-9147
-4966
-7536
-3709
5206
-7924
297
-1732
6710
-669
1975
-8667
8968
7052
-4487
-1529
-6215
-7876
7851
7729
-3909
2526
8542
-6278
2121
-1874
-1193
9938
-8817
6296
-5162
-9651
-8840
23
9260
-6671
-2038
-7762
9259
388
9725
7692
-1415
-2746
-9
-2183
-7491
178
-8764
-9381
-8915
-3074
-2750
-3723
-9047
-2612
2942
4115
2101
-1898
8094
-7065
463
2288
613
898
1562
6455
-3122
3775
4765
-838
-1685
-8668
2519
-3547
1882
-429
-2657
-7991
-8260
8359
-9452
-8848
-6535
3319
-1437
-6246
-7515
4136
4712
-5647
-5443
-5340
2472
-5160
503
-9491
-94
9049
-4710
7549
-8867
-7230
9447
-6497
9289
-4081
-5413
8461
-5899
-6122
-1398
-690
3476
3204
-1493
-7640
-2753
-5032
3590
-4331
-7789
7705
-8704
-2724
-6047
462
8510
4808
7023
1476
-2364
-9980
9593
-673
-9622
-9853
1933
-7653
8089
-553
-6702
9898
5281
8464
-4042
-907
8667
-2301
4210
5398
-2638
-6079
8935
9370
2773
7006
-9314
-8742
-2594
4272
-1463
5798
8007
-4676
";
