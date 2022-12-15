use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;

#[derive(Debug)]
struct RuckSack {
    compartment_1: String,
    compartment_2: String,
}

impl FromStr for RuckSack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let partition = s.len() / 2;
        Ok(RuckSack {
            compartment_1: s[..partition].to_string(),
            compartment_2: s[partition..].to_string(),
        })
    }
}

impl RuckSack {
    fn find_matching_item(&self) -> char {
        self.compartment_1
            .chars()
            .find(|c| self.compartment_2.contains(*c))
            .unwrap()
    }
}

#[derive(Default)]
pub struct Solution {
    item_map: HashMap<char, usize>,
    input: Vec<String>,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.item_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .zip(1..53)
            .collect();
        self.input = input.lines().map(|l| l.to_owned()).collect();
    }

    fn solve_part1(&self) -> String {
        self.input
            .iter()
            .map(|l| l.parse::<RuckSack>().unwrap().find_matching_item())
            .map(|i| self.item_map.get(&i).unwrap())
            .sum::<usize>()
            .to_string()
    }

    fn solve_part2(&self) -> String {
        self.input
            .chunks(3)
            .map(|g| {
                g[0].chars()
                    .find(|c| g[1].contains(*c) && g[2].contains(*c))
                    .unwrap()
            })
            .map(|i| self.item_map.get(&i).unwrap())
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> &'a str {
        "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "157");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "70");
    }
}

const INPUT: &str = "QJRBMDMtRDCtJzBtJMfjNjhwvmNDvwjLVVgh
TPSNNPZGTjgmSmvfjL
bPlpZZbpsTlTsWprpGFCJtRtzMNdMMBBcWnJQB
tppvbQBhpQQdrzMMcLwhMc
gZnWRccRNgFGRGRFRNNgZgJMddddLLLMCPqwLCNPwqPJ
nRRmFSnWmlgZlTlTllSlSWWWTsfvfDQpBfBcpQvpVQpTfQQf
lRlsVFgTlMgRNsSNTlFgmbWnMPppPnMqWZMWPPWW
fDjgBJdCfCHHBnfLWpqnmnpZmf
GjQHHcdvJHQBHSSNsFQFslwwRg
NPwDLDHNwjLLHWjbdSbDfJJQTZsZDS
BcFBcvgFvghnFLrBpvrgcgrJSZJpQdfSTZbCsSdfZZfbCf
VrngVFRmrVWHLGVMlL
SNBBBDlfZDLqNGmgFjjmBsQgCFtF
VPPVbhpbhMhRhncnScRncbrQtCgQQFmjjjsgtRtQHmFQ
nhWcPJVhpbvMvwvwllvSlGlD
wNlNNqtqHHHPhqCz
MMMMcQSWSpQCWFnRRPchLVvPLLzhmhLzhh
CrgRSWrnrQpppRQrCTnRTRtGtBDBfbNBllbTJlZtfNBN
QNbbNrnNnCwHmNPQmzqQNPsCCfBFFGtsBBddBDtCJDJd
gvVgpZWgTWvRvlvLPDDJjGBfdsdpDDJGdd
ZRMWWRMVgRZghTggPSMZzQwwnqwmnzhNnNwHcQHm
VmPHzBmpmQHbVHSpNHBVQCtRPPCPvFFMqqntZCZqMR
dWlDcfcfcjcfDWjlsZfjJhdGvFLGnLsLqsRnvRvRGGRttC
wfJhZTllcfdZdfjJfjdmQzHVSzHzgHQTpHpmpV
qNnqmzmCBfvmDvBm
HcdhtQdttbbhtVcrVVDMfZvdMBTqsWZMBsWZ
HQGtctRblwqpNwRN
SBtBLBMZzPDDNFFDQnVVVnnDmf
dgCjblRdgRvrbwjJGzQQQzwJVJ
WpWbCWWvlgrcCHdvvCdvWbSLZzhhZhtLBPPStSPhMSpM
PlPnGGGzCqqlrqTRsbTmFRWgDPmR
wwpLtjwpzjDwFWRsWTWW
NZtJjHNNhHfnCBcJMBlCSz
wSrwggPrhJhCdddw
tLMNvMTFhDZdhTBh
LtMvFttGbNcWRsLFLsccRRgfnSrPjPnfljSfPWlnPhrS
TSZlwZSSccSHZLHVcllSvmDLmJhjDDffJmGjQjgQQJ
sdBdzsNnBMBstNNMFhNPNbPzgGfDgJrtrfjCjDrCfJmmDQJf
BnnBznRsFRFBsspzzbZpSTqVTpHVhpTvlqVW
VtVjjhdFmCCfhRRzzSDbDzpmgzmvgb
CHJqrswsWvbvJbpD
CqCPcZHGHTcsCBQsBrTGHMFnLVQjMjLVVhdhnFQVRL
tvlPSrlNNvtglTtPccldQdhbQbZdcqqZ
mRmBGHWmDFRsZqHrfbdhqhZZ
jjMGjWrJpttNjtgg
HPtCMJNjvJLMDZRdBgLSBSfsWBgG
VmnrhwwqhbbzrwnDrqpdWBgfdSdfBGgffGWRdh
qmnTFbVnpqVzpnvlDFJZDClNZPZN
NNRFQfzbNWhLHTVh
dGjptnrPqgvqjccvndnnPPhlHrVVTHLWMHlwmrlHMmVTWm
tDggGnPqDcPPpPpddjGhggtJCCSssfJbQsDfbZsbsbRZFQ
bqZWhbvvvqfvhqvQCChhZlllGwlwGjNRrNGrwGwNRQ
PmspSscJVJStzSVzWJlgwlwNlGRLDGrPgNwN
pdHmWMVStWJWFBBCCbMhCfbC
wtwbctGLwGWhGwfWwhNrnLrlrQFNmPNNVrrl
CSdqZRsMStdJMMSZqPnFmVqPQlnjNjqj
TMtsTBSSRZBCJStMJSZTHtfvpgvzzWwhbpwhggbzHpbW
HncMbCwCncHlcbMDMnMFGsNsJVFJGchVTTcmcG
RRfBRNjRLLJTLTThsq
zpBRjWRrRvBpNtRWrgwbrwQPDPMDwCnn
TDcPLTVRjntFwDwDnb
SJJhffHqHZZgHGSFFbdrGTGnGv
NQHWZgJQHNHgHQhlLLLBjpRTjLjMNNLM
sMNnNRNrlGlsZBrGsrFQpclWlWLfpWjtzTfDtpzj
gvhPgwTgdSHtHDtpDPLp
gwhSwdvTSTbSgRrZNrrNFFNBGb
rtZnDHJrrDtGtGHvGHDWfdfwCjcBhjBCffwwLv
lzVlzsTRsmzVNTspVsMMsmwCLcmjmcdbBBChwfBbCW
sVTMpTpppsVMsPRPVzMNFqMFwZtQrHZDGqgHZrSQQrQQJDGn
wGQQMMQvCTPPQnHPBS
FsWdJddszWrrRRJRTmRmpppRHBNPBppNHp
rWdFWlFJzbzzMTwcvvMbGMgc
WTnnTpqSnCLmjGgSgjztgg
rQRHQvbNLwrgtGtrmDglJt
PwHRNvQPsvHvPTTpLTcCLVnq
qTsqJDJHjjfMCSDj
RnGGNFGznzGVnBCWmfSMSLwWRwSj
NnBbVQFVCClctHQc
BHzmfDHfJLGcQBGgQLDcstNttlZgdlltldshgZZg
PwPPSJwPvSNZlvSl
CJwwjnJFFnWRMcMzcHMHRzGL
rmZpvcZcqccsqmqzzzcBRLBZbBBRLBlRGVdfZR
PwjFggwMDgNFwPgTwNFgtFJjfGLhBLsGRGbfBfBLbbTVLbdf
DWJwgDWMJJDWCCNHmrnscmqqcnpWSQ
bsRlVgMhtzHvhRvpzcLSZcTWLGzTTrGc
QJnDjmqjJdmDqqrGWWsZsZTc
nQPsnwCJdBDJDDJvhHhpMRCVlhlgRV
NBNwMCtNgqCHClHClq
JpQmFrQQfHfWjJTfLTjfLRRFRvnvhvnDGDcRcvVGGV
HTzzpzzdHgbBZZtdMB
SWcVvBFBVBjShWhGQtZnHFDHRGGQsR
pMZpmmJPbwbTTQttrDrRrttT
mZflqdlbMVcNjdWLSj
tvjdccdbLjhvhlcjRMvRTCQJmBPBCFRG
qgnqZfHpZDVnCpZzZJQFQBgmmPFJBmRQJQ
SHDZDDzpNVpfsNsHqpDSjLwCbbWLChwtwjtCWc
FsWTbcwmGfFFFrpl
LMhzdfqjLdHQQnSvldGvnS
VZjVNzfNLtjzDMhVDNtqDqwJwRmmmZJgmgcbWgRRwCbJ
ZJbPwwfJcGlwCrrZrMMddMMMtt
pTNvvSSHmmnbpFRp
SLSjSLDSQNLHNDbJbcfJclBzjGsz
WSQCWQWstCWCCgNNsDCZMZDBjjlLPnHMMLPrHlcrcLHHTjTh
bVFJFwfdRFFgjTPgnc
GmzRRqvRddbdRRdfJRJfsqsSSsZDsDBQZtCSgpgt
FPjprPpPCCFpFPHWsWvqnnllQsdLQMMlLtslLQMc
wmzJgzRSRRJghBbwGBBSbtGfLfGlcNnlltdddQtrMd
zDmRBmwDrpVFDTVDVp
FPGqjsZGlDJmzsHcTcTMMs
SQNLSvdbvVbrSbHcftGcrpHGfMmf
CNNGSCCdSCZqjqljZF
GvqpqrpqdqdsdGshSMhhRsSMhhlSlJ
DLCzzjzBwCbQWtQlRRFRRJFptfffgM
WzpLbLDbBcLPjWQWDBzzmnvNndHNqZqZNZNvcrNT
scHCGfWHsvWHVfGsggHfgvVcSLwLLPRwwDLPLllRPDzlPr
tbjqqNNTlPDTTSrD
QntmNbNnnddqJqqbFJHZWWHWJWvZHGVJsSsp
WZjpjwwGBGZQsqBLBHLHSRLP
mJhtdfVtDVJtvVLSmNRSccPPPlNHcH
JJLCFDhLCfVGGwbGGCwrGC
nBnsGSCrptmsLWGhWRvVRJVJ
rllMZZbcWWLJvhTl
MHwzczHwwHqZcdzMdbqSmwsssmtNCrBmtrnQNB
LzwrZNrNzBMrJBzJsfqqntMlVlSfhnhb
HTDPWDHPTgGHWTGcPFRgFpPPtfqmsfqlccmlSmnblbshqnmm
jWGgpRGPFRHjzdBBsrBJvj
hjNghjlwqjzGhwhGwLrMMrsMdsMfczPfsr
ZJQSFZFZpCTQSZHTTFbcWWPbWsWrdLVmrMWMfr
tttHSCpFQBQQpJZSJLgBNNDhqhBqvBvvRq
hLLJJJLcLPLfLwcJDchfhpSmqGbmdQGmGSdbqdbmqGGGdG
zgCCVVvVCNVssdbqmtMWvbnndD
rCCZZCVTjVZNzFZJBlflBLccDhBFFB
wwPPHfCMHQsrcwPbMPMcvQFJvqWgFTZgDFJltgZt
jRBVLhpNqpBmRhhRdNJZJgWTBBtgZWltZJJJ
mSjndhSzphjLRVqmhphNShrGMGrcbGbnGCHGwrwGfbbG
PVBRhBdlwRtRhRBwtBlVzDcGpVcZnggGzGMMsg
fFFWQqbFbLWCWvvFbTjjGnsZMfgsZcZzSZGMpSgD
QJTCCLFFLjFqFbHTbbltmhBNwwcNmthNhlHr
qwPJJsJdbPdwJddQCRCgCTMTRGGwMG
cLFcFBZNWWQLSQRfZjpljTGRCgGR
cFvrcNBFJDhzdQzv
zTsVTqDqQNtNwwMVmN
pHpSzPbRrvbRrGzGMwZwlBJmNtclwJpB
SjHRPfRbffPHqzCCCdTsTzqj
jnbMBnPjjjFtBtMjFPRtGfvvfzgWWHMfWHTlGgHH
dCpdqrVrmdpHfTJTCWGJgG
qVdrppqSTddqNwZcDPPPhZRBPBLBRLjF
VbHqLlGQlgjLjjQsNvCZTsNjMtCZvT
SJtttppwwpwBwdPvsvCvBZrvNrTrvM
JDnWJpDSSpmSwmpPzSwznhDlqGqqtqqHGHLlhblGbR
RqRJJVMPdRVVpqMdFwmvnSMwZcfCGfDSZc
CssQgjssvZvjffmS
zNlbbWTBLWCbCPPFPbVH
nvQsHSsGvNvnQghTRMrrjpjM
ttlLDlzPtGDcRRtpZTFjtgMj
PBLBwPPDzzLwblzffzLlVHHsCCHqsfvCCSsGSNWC
jHrTrThrtHgttThgHTtfgTgsmZZmBSZGSGsSGfZBZFFmQs
qCCPdbcCJddbRcsQSGhFzmZqZGmq
VVNNdVvclDcPbMWMwnnlwhphjp
ZdBgJqFWNNNqnZZNGsBCCCRvrCwCjCssCB
htDPMSPtMPzPTLMzMTMbRRbTbvwRCjfRfsbWWs
LhMmtMDWmHlpppplJZJgNd
mhtsjtbChcpLqmpmzL
DPlPprrfBrpGHHVGNVHRqcNvvLLqLcvJzzTvLc
VFfVPrrBQFPlDDwDwBpBtSgQjnghMhCdbSnnhtMM
DPDMpbsHPDPNtdtrgMtdnQ
WShWlSCJVlzccSBvBvhVZZWlgTNTrNrrQTjQjjjjgDSgSdNt
cvmCDvCJCcsRbmpFmqms
sSfFssmLnLwPtrrmttsFbDvWgCvddVgfgWdRDWlChD
nnGnHBzqHjqBJGChlRClhvghJWDd
jNzNcczMcGntPMwwSsSr
GGPCThCCvCTVWBCBGMVMsTgZJsrZtHNNtrsHJrgH
zjRwcwwfvSjmwznfzQSHDJtgrNrRNrLDsRrHtD
fjvzmcfSlSznwcnmnSQnhdlhWBpGpdBqhGhqhVPd
sHGGqpRqfNRVbDDtVwwzWf
CCLQZllTQLTcSShTQvjhQLnnWrDzVpwtDDwVDnczwMwM
vggZLZTldlhpCTlZlZCRRPNRmqdmGBHPFqsGqN
wwFDFLMDjjCNgNwNlwwgvR
frPbSJMSSPBqrfppSqrBZqMQhHlmNsRZmmslvghsmhsgggtZ
TPSPfBQrdJSfTTqSbbBfTfdcGWjFWWFDWnGMjLjGVFCj
LZRZbHtqnVztHTTTjMBQjQHH
rJcDGpwwgDwCCWFGSFMSffVWfF
cNNNgvhNglDnhdzsbLbmVs
RwmrGVPmNLzdmVpmrVtHDjjgDHHRqjFtngFt
CBlWhQWlTWshsblFGntjHtGbHG
WsTSGZSTQZZJpPNdzSrzwvpr
CVsggSgdwSwghVSTCgVZjJlRvlQNJHJGZVvjvj
qrrnzrrpDFMzbDbbzrMbBcNjRBHHQHGRRllHHPBNBljl
rnFppcpWcqnWMLDNsggSmWmsWfggdg
wjQzPjJcplwmDDBL
vghWhhnfWqzhftWtfnbFBmnGDnLGDbDmmC
zZNvZrNsWfgVftNZhQcSdPHPTcPHQQTTJV
WjvPVbWnbbFvjfLlcplQvLQvCwCl
sJhmrrTRTDDJHhhsmJhmrNDdQwLQQlHllHwwLpCLclBBlcPC
RJTRDdmPmmzNTDhnWtzMfMWtqjqqWM
vvpjqtllDMlHDtDBsPSSfBJFlSffNS
gwTmJrTcJWrNSmsNBBPfmf
VzzJzgTnddzWrwngnWqbHqbtLqjqvpvqhbMd
TlpzwGZGGFmZJdPpRtpHPrpcPs
CMJCMgQjMQvrfMHtMfHv
DjnNjCBqCCNnWWgDBQQDnCZwFJwmwwTznmFVwFmzTJJm
CcDPppDCFdDrFcFsMsdlLVjjLsMHvM
fqSmmtNGqLNffhHHbsMsbjbjNjbv
SthSGmLnmfwfWGWhSQGSQRnGpDpJPCDJrBPTcPrDwPzFcpFT
FdqjDtPWzqPdnPPtPFbssllqLJlqNppsJGppLp
TwfrcvwRgvfTBWRgBssJhspHfffJHlHNGh
MMZCQrrRBwQCCZMQwcTMwPztnFZSDWVWPttPSZzdzd
prHlrpJbdccllrrPbFdrgPzZfZhZVhRZVScNRNWtSZjWRW
LmwCCnvqwGCLMnsWtGRZWVfbfbftRW
bwnvBnLBvbsBvszHzpgBlPzHHlzg
grSJNTSgBHgpqhvCGbbZddGCGbbT
nDLMssQMRLwMtMWRWCZdQfqjfGvZQfCjCc
PqsDWPMLnwlRllJzghmgmSNhpgrl
TQGcWQBDnSzzsBSL
mJJlqJwVJdbSrhlrlhhsLL
JPtwMtdPbJbVqVNpPtmbpwZcQDFFcCccFjCQjpQWSWZg
JfbfpZJmzffmpZnZZwsrwDFvwHPP
RDdQtWTWQQSTGNRhsFsjnvjwrhPjtH
QccddTVQQldcGGRdGlgmVmBzfVpDmbgggmpL
HVnhVcHvpVFWDpmP
QswNZblTTwmqlntDPdqD
sGZzNwsGNThhMrhBBhzn
fQllBlVQncgwLlfWwWDvppZZggZqGpZgpGdvGG
shPTRsFbNFJmvqpGjrpvPDdr
RNFDtRRRssRTStRmTlnzwSVQlVVWfWzcQc
WmCpPCWTjQPCWWSjSTmrqRLGDRFGrTFDRFDLDD
gJnVcnVzdfnZgchvrslMDZGlRRDZLR
fdHhfncwfbfzJbnJzJfcczhhSmLCCNBjSpjmpjHjBQjpmpNW
BDvDPGRwRvCmLssGLmsL
frRjjlldrqtNspLWpqFcCmzm
ndSnVNtllldrdfSjfNvgVRHBwbbVMRbVPJgH
PpgjhpVLghPZhSgZVVzzcJWccPNCrcJzrFsJ
BdBNNMqMdfDnDNTFHHJCqHrJHzrFzF
wfMNtMndlBTlmTBndRpgghhjZRjvSZVjRw
ZQnQMWMcjHDHrWNF
TvtCvvBVgdRdmvBVNzDHlGFjFHjfRfDD
dvtCCbdJmhvhhhhbhVBPMwqZswnZqZjjMccsZJ
DDMzRBBSzRDTMQRZsbvssCbhZtCDtP
dLmwNplnmmwjGvPVCRtVVvVd
NNmjLmqWJjFRwFSrgcrSHBzcTz
TwTwTMBWcWBJJBtTWHddCmfgzlCzClsvmfsM
PPLDnNqPRLQNVnGNVsDQnNmzdhvdddlvdlqgqmdlrfvv
SQQsjPPLGLbDSnGLLNnWTFZJHbcpFctHZpwJWB
FzMltgtMzFpZtmzdjPpnvRTQTvRWTDfnnTlvwW
JcbVcBrqLCVJHJSNCcZVqVqqTRQRWWfNsTfTvDfsWvwTsnwv
rcZqVJVhmhgPmhmd
ttvSnlWvWWgcScMDsHHMPMjPmH
pzLGLfNRpJsvmmfvMDfs
GhpzRqqpZppNrhvFgwSlWnnBFn
sbQcDJQJJDbQhwchSctVnVnqTMvMWSqTMPSMlP
jtjCtNRLNCRgRnlTPPWg
pzpHdLtFNdJbDhJHsQhs
pSqnfqDnWPHNPCCHCp
GdJZQdgZbBvgQLcCZZCCZlPLRH
PzBgQggbvBthtMdMvbzvVfFfzTWqDmWDqzqWrfff
nnJdrfgfrdMCMdgrqMnWdgwNTTTzFhPSSHfSHhllzjzNFT
vBRvmvGZsLZZsHFNFFzTNPzb
LZVRmcDRvpQLmvvVGDGmpntJJwCWCnCPJwgJDrPDqM
QddMvdzlVfvdSQmGhmwLbGbmzbns
JtCCWqqZDsLpGhbGjD
FNrhqCTWMSRSrQQg
ZsBZJFsZSmmJsJSmrJrJrvsrdGdCQGQphMGwRMGQRGdbBChM
FlgfqNNNWnNnHfVnnHdbGwpwGWQhGdRMMdRM
LFnggHlDqDLvjDmZPcPmvP
CRHJWfvJvrQfrCsDlGGBszQBjjGB
LmPHVnMmpLlPssBPlDtd
MmMSZmVnncMFcmSVHvfSrffCwSvfbHWv
wsrJrpdJLsMCZDWL
BbLtGGbNmLQggqgQQtGgMmDCTnWZCZWZTmMmCZnT
qNBGNNgQcbbtGbbFBLVjfcfwHvrHHJHJcr
pCZCpdjBljhjBlpVccCpbDDwRWDsLhLbwDsDwsDw
HNgFSSNvSmdqwsFLFWLGttbw
gMMndNrzNHnzJZVlMCMCTcpc
CfsFNszCrrGzrsggsPfPVNVlqTdSjSqMTdSVTdLL
vRhcHllwJDmnJmDMMdhqSqpVMhdjdp
cvHRvwQBPZZlrQgz
TsFhCtQtQsBBLtBLPvgz
jjWZZjZSMNlNNjljNnlmjjfJLMBGGLvBdzPQpggJJLQzpg
wjbcmmlnQZmlrTsCFVwshwTr
nRGFnFjcdlwLSHSpNNnBfWHN
TgQvPbCMPRhbMPQvtQPvMCRBSHNQHBrQSNfWqpHHrWNWSf
PCgMbPvTZVDgtPRggtCCbgmmFJJLmcGFLjdmJFcDwJmm
dgWPssfdvQCLPLhL
pMtSMtpSmpMpFSMMFZjQCLbLQZZbVbVhNTLblZ
mpqcpzncfWwhzfRf
vntvVnRCsvpBpMjCpTpj
rQdZfhzczNzWcNLTpWgSvjjjpGpMSB
ZqNDQhfcNchLchQqcDqRHJtHVwnwbtvHsbVs
qtJGQgTrqtqQdQDgbGjPzZHWWzVjslPZlG
vBShwRRvvSRSvFvwLSvfcnfBWmHZHVWWHPzlNPWVWjZsWnWV
cLBFBFhCBLlwpFccFBFftqJDQdgdTDJJCbJgCCdg
wfmsPvPwNfvmfLNFvzzJbRMnllhlnLhRLC
gjtqDDTtjgpJcbnMTzCRnCCWhC
SDqtpGSStVtdqpgBVjBGZmFPJNJmffvfPsHZPZQd
HQMBBWrQQmPBvmBWnvrTnMSsbFfcfwgfCgscsmGgwgcJGg
NzzlJLthtlgswGFcwGst
JqNNRqpzhVRWTSQrrvSQ
mFpDZjvmtPPGvFjmmGTzTcFRbHczHTbzQgRS
fNdqhJsNrnnVNhwNVdrdsVczQCcwCMHSTCHgHCRzHgcM
JlgnNhsqVqNqNpPlvZvDDDGlZZ";
