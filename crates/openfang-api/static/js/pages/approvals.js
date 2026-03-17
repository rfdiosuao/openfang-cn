// OpenFang Approvals Page — Execution approval queue for sensitive agent actions
'use strict';

function approvalsPage() {
  return {
    approvals: [],
    filterStatus: 'all',
    loading: true,
    loadError: '',

    get filtered() {
      var f = this.filterStatus;
      if (f === 'all') return this.approvals;
      return this.approvals.filter(function(a) { return a.status === f; });
    },

    get pendingCount() {
      return this.approvals.filter(function(a) { return a.status === 'pending'; }).length;
    },

    async loadData() {
      this.loading = true;
      this.loadError = '';
      try {
        var data = await OpenFangAPI.get('/api/approvals');
        this.approvals = data.approvals || [];
      } catch(e) {
        this.loadError = e.message || 'Could not load approvals.';
      }
      this.loading = false;
    },

    async approve(id) {
      try {
        await OpenFangAPI.post('/api/approvals/' + id + '/approve', {});
        OpenFangToast.success('Approved');
        await this.loadData();
      } catch(e) {
        OpenFangToast.error(e.message);
      }
    },

    async reject(id) {
      var self = this;
      OpenFangToast.confirm('拒绝操作', '您确定要拒绝此操作吗？', async function() {
        try {
          await OpenFangAPI.post('/api/approvals/' + id + '/reject', {});
          OpenFangToast.success('Rejected');
          await self.loadData();
        } catch(e) {
          OpenFangToast.error(e.message);
        }
      });
    },

    timeAgo(dateStr) {
      if (!dateStr) return '';
      var d = new Date(dateStr);
      var secs = Math.floor((Date.now() - d.getTime()) / 1000);
      if (secs < 60) return secs + '秒前';
      if (secs < 3600) return Math.floor(secs / 60) + '分钟前';
      if (secs < 86400) return Math.floor(secs / 3600) + '小时前';
      return Math.floor(secs / 86400) + '天前';
    }
  };
}
